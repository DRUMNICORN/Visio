impl NodiumPlugin for NodiumPluginBrowser {
    fn name(&self) -> String {
        "nodium_plugin_browser".to_string()
    }


    // will create a browser window
    fn windows(&self) -> Vec<Box<dyn NodiumWindow>> {
    }

    fn nodes(&self) -> Vec<NodiumNode> {
        vec![]
    }

    fn services(&self) -> Vec<NodiumService> {
        vec![]
    }
}
