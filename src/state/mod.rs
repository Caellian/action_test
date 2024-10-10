mod issue;

pub enum State {
    Issue(issue::State),
    PullRequest()
}
