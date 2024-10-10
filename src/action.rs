use ghactions::Actions;

#[derive(Actions, Debug, Clone)]
#[action(
    name = "Labeler",
    path = "./action.yml",
    image = "./Dockerfile"
)]
pub struct LabelerAction {

}
