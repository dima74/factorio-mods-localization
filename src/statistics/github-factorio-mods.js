import octokit from '@octokit/rest';
import Case from 'case';

(async () => {
    const api = octokit();
    const params = { q: 'topic:factorio-mod', sort: 'stars', per_page: 100 };
    const response = await api.search.repos(params);
    const repositories = response.data.items;
    const repositoriesNames = repositories.map(repository => repository.name);
    for (const name of repositoriesNames) {
        console.log(Case.capital(name));
    }
})();