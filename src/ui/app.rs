use iced::widget::{column, container, row};
use iced::{Element, Length, Task};

use crate::domain::model::{ConnectionId, DatabaseConnection};
use crate::ui::components::{ConnectionPanel, QueryEditor, ResultView, SchemaView, Sidebar, TabId, TabView};
use crate::ui::messages::{Message, ConnectionFormData};

pub struct DatabaseManagerApp {
    // Components
    sidebar: Sidebar,
    tab_view: TabView,
    connection_panel: ConnectionPanel,
    query_editor: QueryEditor,
    result_view: ResultView,
    schema_view: SchemaView,
    
    // State
    connections: Vec<DatabaseConnection>,
    active_connection: Option<ConnectionId>,
    current_query: String,
    
    // UI State
    connection_form: ConnectionFormData,
    show_connection_dialog: bool,
    is_executing_query: bool,
}

impl DatabaseManagerApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                sidebar: Sidebar::new(),
                tab_view: TabView::new(),
                connection_panel: ConnectionPanel::new(),
                query_editor: QueryEditor::new(),
                result_view: ResultView::new(),
                schema_view: SchemaView::new(),
                connections: Vec::new(),
                active_connection: None,
                current_query: String::new(),
                connection_form: ConnectionFormData::default(),
                show_connection_dialog: false,
                is_executing_query: false,
            },
            Task::none()
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Initialize => {
                Task::none()
            }
            
            // Tab navigation
            Message::TabChanged(tab_index) => {
                if let Some(tab_id) = TabId::all().get(tab_index) {
                    self.tab_view.set_active_tab(*tab_id);
                }
                Task::none()
            }
            
            // Sidebar navigation
            Message::SidebarToggled => {
                self.sidebar.toggle();
                Task::none()
            }
            
            Message::NavigateToConnections => {
                self.tab_view.set_active_tab(TabId::Connections);
                Task::none()
            }
            
            Message::NavigateToQueryEditor => {
                self.tab_view.set_active_tab(TabId::QueryEditor);
                Task::none()
            }
            
            Message::NavigateToResults => {
                self.tab_view.set_active_tab(TabId::Results);
                Task::none()
            }
            
            Message::NavigateToSchema => {
                self.tab_view.set_active_tab(TabId::Schema);
                Task::none()
            }
            
            Message::NavigateToHistory => {
                self.tab_view.set_active_tab(TabId::History);
                Task::none()
            }
            
            // Connection management
            Message::NewConnectionRequested => {
                self.show_connection_dialog = true;
                self.connection_form = ConnectionFormData::default();
                self.tab_view.set_active_tab(TabId::Connections);
                Task::none()
            }
            
            Message::ConnectionFormUpdated(form_data) => {
                self.connection_form = form_data;
                Task::none()
            }
            
            Message::ConnectionRequested(connection_id) => {
                self.active_connection = Some(connection_id);
                // Here you would typically establish the actual database connection
                println!("Connecting to database with ID: {:?}", connection_id);
                Task::none()
            }
            
            Message::ConnectionEditRequested(connection_id) => {
                // Load connection data into form for editing
                if let Some(connection) = self.connections.iter().find(|c| c.id == connection_id) {
                    self.connection_form = ConnectionFormData {
                        name: connection.name.clone(),
                        host: connection.host.clone(),
                        port: connection.port.to_string(),
                        database: connection.database.clone(),
                        username: connection.username.clone(),
                        password: String::new(), // Don't pre-fill password for security
                        connection_type: connection.connection_type.clone(),
                    };
                    self.show_connection_dialog = true;
                }
                Task::none()
            }
            
            Message::ConnectionDeleteRequested(connection_id) => {
                self.connections.retain(|c| c.id != connection_id);
                if self.active_connection == Some(connection_id) {
                    self.active_connection = None;
                }
                Task::none()
            }
            
            // Query editor
            Message::QueryTextChanged(text) => {
                self.current_query = text.clone();
                self.query_editor.update_query(text);
                Task::none()
            }
            
            Message::QueryExecuteRequested => {
                if self.active_connection.is_some() && !self.current_query.trim().is_empty() {
                    self.is_executing_query = true;
                    self.query_editor.set_executing(true);
                    self.tab_view.set_active_tab(TabId::Results);
                    
                    // Here you would execute the actual query
                    println!("Executing query: {}", self.current_query);
                    
                    // Simulate query execution
                    Task::perform(
                        async { 
                            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                            "Query executed successfully".to_string()
                        },
                        Message::QueryExecuted
                    )
                } else {
                    Task::none()
                }
            }
            
            Message::QueryExecuted(result) => {
                self.is_executing_query = false;
                self.query_editor.set_executing(false);
                
                // Create mock result data
                let mock_results = vec![
                    vec!["ID".to_string(), "Name".to_string(), "Email".to_string()],
                    vec!["1".to_string(), "John Doe".to_string(), "john@example.com".to_string()],
                    vec!["2".to_string(), "Jane Smith".to_string(), "jane@example.com".to_string()],
                ];
                
                self.result_view.set_results(mock_results);
                println!("Query result: {}", result);
                Task::none()
            }
            
            Message::QuerySaveRequested => {
                // Save current query
                println!("Saving query: {}", self.current_query);
                Task::none()
            }
            
            Message::QueryLoadRequested => {
                // Load saved query
                println!("Loading query...");
                Task::none()
            }
            
            Message::QueryCleared => {
                self.current_query.clear();
                self.query_editor.clear_query();
                Task::none()
            }
            
            Message::QueryFormatRequested => {
                // Format the SQL query
                println!("Formatting query...");
                Task::none()
            }
            
            Message::QueryExplainRequested => {
                // Explain query execution plan
                println!("Explaining query...");
                Task::none()
            }
            
            Message::QueryHistoryRequested => {
                self.query_editor.toggle_history();
                Task::none()
            }
            
            Message::QueryFromHistorySelected(_history_id) => {
                // Load query from history
                println!("Loading query from history...");
                Task::none()
            }
            
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let main_content = match self.tab_view.active_tab() {
            TabId::Connections => self.connection_panel.view(),
            TabId::QueryEditor => self.query_editor.view(),
            TabId::Results => self.result_view.view(),
            TabId::Schema => self.schema_view.view(),
            TabId::History => self.query_editor.view(), // For now, show query editor for history
        };

        let content_area = column![
            self.tab_view.view(),
            main_content
        ]
        .spacing(0);

        let main_layout = row![
            self.sidebar.view(),
            content_area
        ]
        .spacing(0);

        container(main_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Default for DatabaseManagerApp {
    fn default() -> Self {
        Self::new().0
    }
}