import crowdin from "./crowdin";

class Main {
    onRepositoriesAdded(repositories) {
        for (const repository of repositories) {
            this.onRepositoryAdded(repository);
        }
    }

    onRepositoryAdded(repository) {
        console.log(repository.id, repository.full_name);
        crowdin.
    }
}

const main = new Main();
export default main;
