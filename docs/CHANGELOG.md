# Changelog

<a name="1.0.6"></a>
## 1.0.6 (2022-06-28)

### Features

- ✨ Separate PUT requests into two different requests. [[873f016](https://github.com/FoamFactory/archetype/commit/873f01665f6dc01383d4b2eb277c46cc772a946a)]


<a name="0.6.0"></a>
## 0.6.0 (2022-06-20)

### Features

- ✅ Add setup script for testing locally. [[7cac0b9](https://github.com/FoamFactory/archetype/commit/7cac0b9028ff5f811ba1f9c70dccd6570ae157ff)]

### Performance Improvements

- ⚡ Enforce a 2MB limit for file sizes being uploaded. [[2c84730](https://github.com/FoamFactory/archetype/commit/2c84730c5f32c619a44029ae4abec515efa063df)]

### Documentation and Copy Changes

- 📖 Update documentation to reflect new limits. [[7dc3650](https://github.com/FoamFactory/archetype/commit/7dc365043ff66c5d2e0782a84547510e975a648b)]

### Dependency Changes

- ⬆️ Update Rocket to 0.5.0-rc.2. [[a461500](https://github.com/FoamFactory/archetype/commit/a461500cbd784504a99cafdaf6b8e0d229ec0533)]

### Miscellaneous

-  Merge branch &#x27;jwir3/fixes-for-file-size&#x27; [[e6f2e4f](https://github.com/FoamFactory/archetype/commit/e6f2e4f049cb690dd7a8562f404c8899729f7d93)]


<a name="1.0.5"></a>
## 1.0.5 (2022-04-10)

### Performance Improvements

- ⚡ Remove data uris and images from avatar listing. [[29ca864](https://github.com/FoamFactory/archetype/commit/29ca864ba852c6888a5060266452b0f08836441f)]

### Documentation and Copy Changes

- 📖 Update README documentation. [[6f796af](https://github.com/FoamFactory/archetype/commit/6f796af3f7c4584d2f591cce3f72eee0e8bd3409)]

### Dependency Changes

- ⬆️ Bump version number in preparation for release. [[f3332c8](https://github.com/FoamFactory/archetype/commit/f3332c8508d440f22fb7178abaaa8b21fc61afa1)]

### Refactorings

- 🔥 Remove const_fn_trait_bound feature since it&#x27;s in stable. [[2eb6fd7](https://github.com/FoamFactory/archetype/commit/2eb6fd7f59abf081e6fe62d88c4e1aeda6adedf9)]

### Miscellaneous

-  Merge branch &#x27;jwir3/[#7](https://github.com/FoamFactory/archetype/issues/7)-dehydrated-listing&#x27; [[51ccf3f](https://github.com/FoamFactory/archetype/commit/51ccf3f2cb00056522df56a76105ba9461aee100)]


<a name="1.0.4"></a>
## 1.0.4 (2022-04-10)

### Features

- ✨ Split upload_avatar into two methods. [[f157cc8](https://github.com/FoamFactory/archetype/commit/f157cc8c45a540585fb5e890894cb1a8fcc4e1cf)]

### Bug Fixes

- 🐛 Fix issues related to building docker images. [[0964586](https://github.com/FoamFactory/archetype/commit/096458605232439fbaa4b3fc84ddba9ea4004f81)]

### Documentation and Copy Changes

- 📖 Add API documentation for new POST method for avatars. [[0abf330](https://github.com/FoamFactory/archetype/commit/0abf33059b9b940aa77abcd1cff6de532fb7e5d4)]
- 📖 Update documentation and Docker configuration files. [[1cf6f9d](https://github.com/FoamFactory/archetype/commit/1cf6f9dab3a33a219b5c73ab1491dad3d3717ac8)]

### Data Structure and Configuration Changes

- 🔧 Ignore test environment variable file. [[4ae057c](https://github.com/FoamFactory/archetype/commit/4ae057c18a31fb027216aa9b55c265d2172c0fad)]

### Dependency Changes

- ⬆️ Bump version number in preparation for release. [[c58c634](https://github.com/FoamFactory/archetype/commit/c58c63412f8437260978629e39cfd6026f79ef0c)]

### Refactorings

- 🥅 Add a catcher for 404 errors that returns JSON. [[086b245](https://github.com/FoamFactory/archetype/commit/086b245f59faeaf7cbf05f0adfa55f45003925f4)]

### Miscellaneous

-  Merge branch &#x27;jwir3/add-data-uri-upload&#x27; [[5cda0fc](https://github.com/FoamFactory/archetype/commit/5cda0fcbc1aa49236b50e7850c6cdb7cc0e46a4f)]


<a name="1.0.2"></a>
## 1.0.2 (2022-03-06)

### Features

- ✨ Make allowed hosts a regular expression. [[c1d8e8f](https://github.com/FoamFactory/archetype/commit/c1d8e8f81343afc7f55a8eddd46dbacf20f06895)]
- ✨ Add update operation using PUT. [[dfccc47](https://github.com/FoamFactory/archetype/commit/dfccc47c68cc9e68361204d2603cc9c5e9e8cc90)]
- ✨ Add a delete endpoint. [[e919982](https://github.com/FoamFactory/archetype/commit/e91998288929a406c2ca5064e3969d0529b3d365)]
- ✨ Add the ability to create an avatar via POST. [[1607da2](https://github.com/FoamFactory/archetype/commit/1607da225d28578d7c96c7a13e37ef8a0e334d13)]
- ✅ Add tests for current behavior. [[74def78](https://github.com/FoamFactory/archetype/commit/74def78629b473a8bb94a2cf0b6461d3eefa17f1)]
- ✨ Add ability to retrieve avatars from SQLite database. [[bfa3d1d](https://github.com/FoamFactory/archetype/commit/bfa3d1dd33dcf0fa7b26a4d558bfac5d48cd92ac)]
- 🎉 Initial commit. [[2285da5](https://github.com/FoamFactory/archetype/commit/2285da5715ed12f4f25cec255f1086cb14c91fa6)]

### Bug Fixes

- 🐛 Fix issue where data would not persist after docker compose runs. [[a6d7b09](https://github.com/FoamFactory/archetype/commit/a6d7b0988ba0424caa5a823dc14e0f507ba64d13)]
- 🐛 Fix issue where archetype would return a 500 when id was not found. [[9dd73ed](https://github.com/FoamFactory/archetype/commit/9dd73ede10ca211e7f2785635ad0baeb761a246e)]

### Documentation and Copy Changes

- 📖 Update README and make some quality of life improvements. [[97e9f7f](https://github.com/FoamFactory/archetype/commit/97e9f7febfcec6b62aa0430605566dff063194ae)]
- 📖 Update API documentation. [[f0dfd1d](https://github.com/FoamFactory/archetype/commit/f0dfd1d3fd24594686af1d707f5bfad8e4fee74c)]

### Data Structure and Configuration Changes

- 🔧 Update docker compose in prep for image push. [[fb8073f](https://github.com/FoamFactory/archetype/commit/fb8073f3f3d36fa41912f505bfa5e2aa309f5dbf)]
- 🔧 Make docker forward port 3307 for the mysql port. [[59be067](https://github.com/FoamFactory/archetype/commit/59be0672510b68cf50af27bfe4373dde05c680e0)]
- 🗃️ Change database to MySQL instead of Sqlite. [[39c4ffc](https://github.com/FoamFactory/archetype/commit/39c4ffcc5a5554f768440e80afcbb0ec33dc26b8)]

### Refactorings

- ♻️ Separate all business logic into a library. [[b19cc6c](https://github.com/FoamFactory/archetype/commit/b19cc6caaab6df8059973046c68bce9d962d2f3e)]
- 🥅 Catch 403 errors and re-send as JSON for clarity [[a4d3152](https://github.com/FoamFactory/archetype/commit/a4d315242906c9eb59466fcf31a0b7ea36205282)]
- ♻️ DRY up JSON conversion code. [[43c007b](https://github.com/FoamFactory/archetype/commit/43c007b9ce8f14a6e7f02e3aa6ca7c5f2936ee6e)]

### Miscellaneous

-  Merge pull request [#6](https://github.com/FoamFactory/archetype/issues/6) from FoamFactory/jwir3/[#3](https://github.com/FoamFactory/archetype/issues/3)-persistence [[f933b67](https://github.com/FoamFactory/archetype/commit/f933b6767aab78d9133c75e9cad9555a572e7fff)]
-  :robot: Adjust docker setup to work via localhost. [[501e44f](https://github.com/FoamFactory/archetype/commit/501e44f4927f89e0e78f8cee59e309fa2ab2c648)]
- 📦 Add docker compose file and Dockerfile. [[4e6b861](https://github.com/FoamFactory/archetype/commit/4e6b86163853a3d7c4c355c48f3e7fd9e5538f04)]


<a name="0.6.0"></a>
## 0.6.0 (2022-06-20)

### Features

- ✅ Add setup script for testing locally. [[7cac0b9](https://github.com/FoamFactory/archetype/commit/7cac0b9028ff5f811ba1f9c70dccd6570ae157ff)]

### Performance Improvements

- ⚡ Enforce a 2MB limit for file sizes being uploaded. [[2c84730](https://github.com/FoamFactory/archetype/commit/2c84730c5f32c619a44029ae4abec515efa063df)]

### Documentation and Copy Changes

- 📖 Update documentation to reflect new limits. [[7dc3650](https://github.com/FoamFactory/archetype/commit/7dc365043ff66c5d2e0782a84547510e975a648b)]

### Dependency Changes

- ⬆️ Update Rocket to 0.5.0-rc.2. [[a461500](https://github.com/FoamFactory/archetype/commit/a461500cbd784504a99cafdaf6b8e0d229ec0533)]

### Miscellaneous

-  Merge branch &#x27;jwir3/fixes-for-file-size&#x27; [[e6f2e4f](https://github.com/FoamFactory/archetype/commit/e6f2e4f049cb690dd7a8562f404c8899729f7d93)]


