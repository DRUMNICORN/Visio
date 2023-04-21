// a enum with all the events that can be emitted by the app
// it will be used to register the events in the event bus
// and can be make into string 

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodiumEventType {
    // the app is ready
    AddPlugin,
    RemovePlugin,
    ReloadPlugins,

}

// a struct that will be used to send the event to the event bus
// it will be used to send the event to the event bus
// into string

impl ToString for NodiumEventType {
    fn to_string(&self) -> String {
        match self {
            NodiumEventType::AddPlugin => "AddPlugin",
            NodiumEventType::RemovePlugin => "RemovePlugin",
            NodiumEventType::ReloadPlugins => "ReloadPlugins",
        }
        .to_string()
    }
}