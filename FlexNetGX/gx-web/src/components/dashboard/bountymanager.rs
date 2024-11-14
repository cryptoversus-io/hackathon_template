// FlexNetGX/gx-web/src/components/dashboard/moderate.rs
use yew::prelude::*;
use crate::components::gx-mobile::{Card, DataGrid, Chart, Tabs, Button};
use crate::components::moderate::{bountyManager, DataVisualization, TeamCollaboration};
use crate::services::moderate::moderateService;
use crate::types::{moderateData, bounty, Analysis, TeamMember};

pub struct moderateDashboard {
    link: ComponentLink<Self>,
    moderate_service: moderateService,
    current_data: Option<moderateData>,
    active_bounties: Vec<bounty>,
    team_members: Vec<TeamMember>,
    analyses: Vec<Analysis>,
    loading: bool,
    error: Option<String>,
}

pub enum Msg {
    FetchData,
    DataReceived(moderateData),
    Createbounty(bounty),
    UpdateAnalysis(Analysis),
    ExportData(String),
    ShareAnalysis(Analysis, Vec<String>),
    Error(String),
    ClearError,
}

impl Component for moderateDashboard {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut dashboard = Self {
            link,
            moderate_service: moderateService::new(),
            current_data: None,
            active_bounties: vec![],
            team_members: vec![],
            analyses: vec![],
            loading: true,
            error: None,
        };
        
        dashboard.link.send_message(Msg::FetchData);
        dashboard
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                self.loading = true;
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match moderateService::fetch_moderate_data().await {
                        Ok(data) => link.send_message(Msg::DataReceived(data)),
                        Err(e) => link.send_message(Msg::Error(e.to_string())),
                    }
                });
                false
            }
            Msg::DataReceived(data) => {
                self.current_data = Some(data);
                self.loading = false;
                true
            }
            Msg::Createbounty(bounty) => {
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match moderateService::create_bounty(bounty).await {
                        Ok(_) => link.send_message(Msg::FetchData),
                        Err(e) => link.send_message(Msg::Error(e.to_string())),
                    }
                });
                false
            }
            // Other message handlers...
            _ => false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="flex flex-col h-full">
                { self.view_header() }
                <div class="flex-grow p-6">
                    <Tabs>
                        <div label="Dashboard">
                            { self.view_dashboard_content() }
                        </div>
                        <div label="bounties">
                            { self.view_bounties() }
                        </div>
                        <div label="Analysis">
                            { self.view_analysis() }
                        </div>
                        <div label="Team">
                            { self.view_team() }
                        </div>
                    </Tabs>
                </div>
            </div>
        }
    }
}

impl moderateDashboard {
    fn view_header(&self) -> Html {
        html! {
            <header class="bg-white shadow-md p-4">
                <div class="flex justify-between items-center">
                    <h1 class="text-2xl font-bold text-gray-800">{"moderate Dashboard"}</h1>
                    <div class="flex space-x-4">
                        <Button
                            variant="primary"
                            onclick=self.link.callback(|_| Msg::Createbounty(bounty::default()))
                        >
                            {"New bounty"}
                        </Button>
                        <Button
                            variant="secondary"
                            onclick=self.link.callback(|_| Msg::ExportData("csv".to_string()))
                        >
                            {"Export Data"}
                        </Button>
                    </div>
                </div>
            </header>
        }
    }

    fn view_dashboard_content(&self) -> Html {
        html! {
            <div class="grid grid-cols-12 gap-6">
                { self.view_stats_cards() }
                { self.view_recent_activity() }
                { self.view_data_visualization() }
            </div>
        }
    }

    fn view_stats_cards(&self) -> Html {
        if let Some(data) = &self.current_data {
            html! {
                <div class="col-span-12 grid grid-cols-4 gap-4">
                    <Card>
                        <div class="p-4">
                            <h3 class="text-lg font-semibold text-gray-600">{"Active bounties"}</h3>
                            <p class="text-3xl font-bold">{ data.active_bounties_count }</p>
                        </div>
                    </Card>
                    <Card>
                        <div class="p-4">
                            <h3 class="text-lg font-semibold text-gray-600">{"Total Responses"}</h3>
                            <p class="text-3xl font-bold">{ data.total_responses }</p>
                        </div>
                    </Card>
                    <Card>
                        <div class="p-4">
                            <h3 class="text-lg font-semibold text-gray-600">{"Team Members"}</h3>
                            <p class="text-3xl font-bold">{ data.team_member_count }</p>
                        </div>
                    </Card>
                    <Card>
                        <div class="p-4">
                            <h3 class="text-lg font-semibold text-gray-600">{"Analysis Reports"}</h3>
                            <p class="text-3xl font-bold">{ data.analysis_count }</p>
                        </div>
                    </Card>
                </div>
            }
        } else {
            html! {
                <div class="col-span-12">
                    <div class="animate-pulse flex space-x-4">
                        // Loading placeholders...
                    </div>
                </div>
            }
        }
    }

    fn view_data_visualization(&self) -> Html {
        html! {
            <div class="col-span-8">
                <Card>
                    <div class="p-4">
                        <h2 class="text-xl font-bold mb-4">{"Data Visualization"}</h2>
                        <DataVisualization
                            data=self.current_data.clone()
                            onupdate=self.link.callback(Msg::UpdateAnalysis)
                        />
                    </div>
                </Card>
            </div>
        }
    }

    fn view_bounties(&self) -> Html {
        html! {
            <div class="space-y-6">
                <div class="flex justify-between items-center">
                    <h2 class="text-xl font-bold">{"Active bounties"}</h2>
                    <Button
                        variant="primary"
                        onclick=self.link.callback(|_| Msg::Createbounty(bounty::default()))
                    >
                        {"Create New bounty"}
                    </Button>
                </div>
                <bountyManager
                    bounties=self.active_bounties.clone()
                    onupdate=self.link.callback(|_| Msg::FetchData)
                />
            </div>
        }
    }
}