use ghactions::Actions;

#[derive(Actions, Debug, Clone)]
#[action(
    name = "context-label",
    description = "Contextually aware automatic labeling",
    path = "./action.yml",
    image = "./Dockerfile"
)]
pub struct LabelerAction {

}
