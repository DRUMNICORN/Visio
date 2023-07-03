use std::sync::Arc;
use tokio::sync::Mutex;

use nodium_app::{NodiumApp, flow::NodiumFlow};

pub async fn handle_plugins_list(app: &Arc<Mutex<NodiumApp>>) {
    let app_locked = app.lock().await;
    let plugins = app_locked.plugins.lock().await.get_plugins().await;

    println!("Plugins: {:?}", plugins);
    for plugin in plugins {
        println!("- {}", plugin);
    }
}

pub async fn handle_plugins_reload(app: &Arc<Mutex<NodiumApp>>) {
    println!("Starting reload");
    let app_locked = app.lock().await;
    app_locked.plugins.lock().await.reload().await;
    println!("Finished reload");
}

pub async fn handle_plugins_rebuild(app: &Arc<Mutex<NodiumApp>>) {
    println!("Starting rebuild");
    let app_locked = app.lock().await;
    app_locked.plugins.lock().await.rebuild().await;
    println!("Finished rebuild");
}

pub async fn handle_flow_list(app: &Arc<Mutex<NodiumApp>>) {
    println!("Handle flow list");
    let app_locked = app.lock().await;
    let flows = app_locked.get_flows().await;
    let flows_locked = flows.lock().await;
    let flow_values: Vec<&NodiumFlow> = flows_locked.values().collect();
    for flow in flow_values {
        println!("{:?}", flow);
    }
    
}
pub async fn handle_flow_add(app: &Arc<Mutex<NodiumApp>>, args: Vec<String>) {
    if args.is_empty() {
        println!("Error: You must provide a name for the flow.");
        // TODO: Handle error
        return;
    }

    let flow_name = if args.len() == 1 {
        args[0].to_string()
    } else {
        args.join(" ")
    };

    let app_clone = Arc::clone(&app);

    handle_flow_add_impl(&app_clone, flow_name).await
}

async fn handle_flow_add_impl(app: &Arc<Mutex<NodiumApp>>, flow_name: String) {
    let mut app_locked = app.lock().await;
    app_locked.add_flow(flow_name).await;
}


pub async fn handle_flow_remove(app: &Arc<Mutex<NodiumApp>>, flow_name: String) {
    let mut app_locked = app.lock().await;
    app_locked.remove_flow(&flow_name).await;
    println!("Flow {} removed", flow_name);
}