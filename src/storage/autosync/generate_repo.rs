use github::{repos, GithubClient, RepositoryMetadata};
use crate::storage::{USER_DATA_REPO_NAME, DATA_REPO_TOPIC};

// Create the homebrew repo on the github client viewer (the user that is logged in).
// https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#create-a-repository-for-the-authenticated-user
// https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#create-or-update-file-contents
pub struct GenerateDataRepo {
	pub status: super::Status,
	pub client: GithubClient,
}
impl GenerateDataRepo {
	pub async fn run(self) -> Result<(), github::Error> {
		self.status.push_stage("Initializing Storage", None);

		let create_repo = repos::create::Args {
			org: None,
			name: USER_DATA_REPO_NAME,
			private: true,
		};
		let owner = self.client.create_repo(create_repo).await?;

		let set_topics = repos::set_topics::Args {
			owner: owner.as_str(),
			repo: USER_DATA_REPO_NAME,
			topics: vec![DATA_REPO_TOPIC.to_owned()],
		};
		self.client.set_repo_topics(set_topics).await?;

		self.status.pop_stage();
		Ok(())
	}
}
