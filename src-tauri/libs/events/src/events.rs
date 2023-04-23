// a enum with all the events that can be emitted by the app
// it will be used to register the events in the event bus
// and can be make into string 

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// its strings
pub enum NodiumEventType { 
    // the app is ready
    // AddPlugin,
    // RemovePlugin,
    PluginsReload,
    PluginInstall,

}

// a struct that will be used to send the event to the event bus
// it will be used to send the event to the event bus
// into string

impl ToString for NodiumEventType {
    fn to_string(&self) -> String {
        match self {
            NodiumEventType::PluginsReload => "PluginsReload",
            NodiumEventType::PluginInstall => "PluginInstall",
        }
        .to_string()
    }
}