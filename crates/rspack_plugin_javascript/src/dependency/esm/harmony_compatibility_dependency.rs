use rspack_core::{
  DependencyTemplate, InitFragmentKey, InitFragmentStage, NormalInitFragment, RuntimeGlobals,
  TemplateContext, TemplateReplaceSource,
};

// Mark module `__esModule`.
// Add `__webpack_require__.r(__webpack_exports__);`.
#[derive(Debug, Clone)]
pub struct HarmonyCompatibilityDependency;

impl DependencyTemplate for HarmonyCompatibilityDependency {
  fn apply(
    &self,
    _source: &mut TemplateReplaceSource,
    code_generatable_context: &mut TemplateContext,
  ) {
    let TemplateContext {
      runtime_requirements,
      init_fragments,
      compilation,
      module,
      ..
    } = code_generatable_context;
    let mgm = compilation
      .module_graph
      .module_graph_module_by_identifier(&module.identifier())
      .expect("should have mgm");
    // TODO __esModule is used
    runtime_requirements.insert(RuntimeGlobals::MAKE_NAMESPACE_OBJECT);
    runtime_requirements.insert(RuntimeGlobals::EXPORTS);
    init_fragments.push(Box::new(NormalInitFragment::new(
      format!(
        "{}({});\n",
        RuntimeGlobals::MAKE_NAMESPACE_OBJECT,
        mgm.get_exports_argument()
      ),
      InitFragmentStage::StageHarmonyExports,
      0,
      InitFragmentKey::HarmonyCompatibility,
      None,
    )));

    if matches!(
      compilation.module_graph.is_async(&module.identifier()),
      Some(true)
    ) {
      runtime_requirements.insert(RuntimeGlobals::MODULE);
      runtime_requirements.insert(RuntimeGlobals::ASYNC_MODULE);
      init_fragments.push(Box::new(NormalInitFragment::new(
        format!(
          "{}({}, async function (__webpack_handle_async_dependencies__, __webpack_async_result__) {{ try {{\n",
          RuntimeGlobals::ASYNC_MODULE,
          compilation
            .module_graph
            .module_graph_module_by_identifier(&module.identifier())
            .expect("should have mgm")
            .get_module_argument()
        ),
        InitFragmentStage::StageAsyncBoundary,
        0,
        InitFragmentKey::unique(),
        Some(format!("\n__webpack_async_result__();\n}} catch(e) {{ __webpack_async_result__(e); }} }}{});", if matches!(mgm.build_meta.as_ref().map(|meta| meta.has_top_level_await), Some(true)) { ", 1" } else { "" })),
      )));
    }
  }
}
