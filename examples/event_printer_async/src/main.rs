use swayipc::{block_on, Connection, EventType, Fallible};

fn main() -> Fallible<()> {
    block_on(async {
        let subs = [
            EventType::Workspace,
            EventType::Shutdown,
            EventType::Mode,
            EventType::Window,
            EventType::BarConfigUpdate,
            EventType::Binding,
        ];
        let mut events = Connection::new().await?.subscribe(&subs).await?;
        loop {
            let event = events.next().await?;
            println!("{:?}\n", event)
        }
    })
}
