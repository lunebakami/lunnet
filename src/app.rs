use networkmanager::{
    devices::{Any, Device, Wireless},
    Error, NetworkManager,
};

pub enum CurrentScreen {
    Main,
    Exiting,
}

pub struct App<'a> {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub network_manager: NetworkManager<'a>,
}

impl<'a> App<'a> {
    pub fn new(nm: NetworkManager<'a>) -> Result<App<'a>, Box<dyn std::error::Error>> {
        Ok(App {
            current_screen: CurrentScreen::Main,
            network_manager: nm,
        })
    }

    pub fn get_active_connection_name(&self) -> Result<String, Error> {
        let nm = &self.network_manager;

        let mut conn: Option<String> = None;
        for dev in nm.get_devices()? {
            match dev {
                Device::WiFi(x) => {
                    let active_connection = x.active_connection()?;

                    conn = match active_connection.id() {
                        Ok(conn_name) => Some(conn_name),
                        Err(_) => None,
                    }
                }
                _ => {}
            }
        }

        match conn {
            Some(conn_name) => Ok(conn_name),
            None => Ok("".to_string()),
        }
    }

    pub fn get_closest_wifis(&self) -> Result<String, Error> {
        let nm = &self.network_manager;

        let mut connections: Vec<String> = Vec::new();
        for dev in nm.get_devices()? {
            match dev {
                Device::WiFi(x) => {
                    let active_connection = x.active_connection()?.id()?;

                    let access_points = x.get_access_points()?;

                    access_points.iter().for_each(|f| {
                        let access_p = f.ssid().unwrap();
                        let strength = f.strength().unwrap();
                        let mut is_connected = "";

                        if active_connection == access_p {
                            is_connected = " Conectada";
                        }

                        // TODO: change this to add to connections vec
                        if strength > 60 {
                            println!(
                                "Name: {:?}, Strength: {:?} {}",
                                access_p, strength, is_connected
                            );
                        };

                        ()
                    })
                }
                _ => {}
            }
        }

        Ok(())
    }
}
