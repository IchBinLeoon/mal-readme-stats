<h1 align="center">MyAnimeList Readme Stats</h1>

<p align="center">Dynamically generated MyAnimeList stats for your profile readme</p>

<p align="center">
  <a href="https://github.com/IchBinLeoon/mal-readme-stats/deployments/Production">
    <img src="https://img.shields.io/github/deployments/IchBinLeoon/mal-readme-stats/Production?style=flat-square&label=production" alt="Production">
  </a>
  <a href="https://github.com/IchBinLeoon/mal-readme-stats/commits/main">
    <img src="https://img.shields.io/github/last-commit/IchBinLeoon/mal-readme-stats?style=flat-square" alt="Commit">
  </a>
  <a href="https://github.com/IchBinLeoon/mal-readme-stats/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/IchBinLeoon/mal-readme-stats?style=flat-square" alt="License">
  </a>
</p>

<p align="center">
  <a href="#-Why">Why? 🤔</a>
  •
  <a href="#-Usage">Usage 🚀</a>
  •
  <a href="#-Deploy-your-own">Deploy your own 🔧</a>
  •
  <a href="#-Contribute">Contribute 🤝</a>
  •
  <a href="#-License">License 📜</a>
</p>

## 🤔 Why?
Because your GitHub profile deserves more than just commit streaks. Why flex your coding skills when you can flex your superior taste in anime?

## 🚀 Usage
Replace `[media]` with either `anime` or `manga` and `[user]` with your MyAnimeList username.

### Statistics
```
[![Statistics](https://mal-readme-stats.vercel.app/api/statistics/[media]/[user])](https://github.com/IchBinLeoon/mal-readme-stats)
```

<details>
<summary>Show anime example</summary>
<br>

[![Anime Statistics](https://mal-readme-stats.vercel.app/api/statistics/anime/IchBinLeoon)](https://github.com/IchBinLeoon/mal-readme-stats)

</details>

<details>
<summary>Show manga example</summary>
<br>

[![Manga Statistics](https://mal-readme-stats.vercel.app/api/statistics/manga/IchBinLeoon)](https://github.com/IchBinLeoon/mal-readme-stats)

</details>

### Activity
```
[![Activity](https://mal-readme-stats.vercel.app/api/activity/[media]/[user])](https://github.com/IchBinLeoon/mal-readme-stats)
```

You can pass the query parameter `?limit=` to specify the number of entries. The minimum is 1, the maximum is 10, and the default is 5.

<details>
<summary>Show anime example</summary>
<br>

[![Anime Activity](https://mal-readme-stats.vercel.app/api/activity/anime/IchBinLeoon)](https://github.com/IchBinLeoon/mal-readme-stats)

</details>

<details>
<summary>Show manga example</summary>
<br>

[![Manga Activity](https://mal-readme-stats.vercel.app/api/activity/manga/IchBinLeoon)](https://github.com/IchBinLeoon/mal-readme-stats)

</details>

## 🔧 Deploy your own

### Vercel

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https%3A%2F%2Fgithub.com%2FIchBinLeoon%2Fmal-readme-stats&env=CLIENT_ID&envDescription=MyAnimeList%20Application%20Client%20ID)

<details>
<summary>Step-by-step guide</summary>
<br>

1. Register or log in to [Vercel](https://vercel.com).
2. Fork this repository and import it.
3. Create a MyAnimeList application [here](https://myanimelist.net/apiconfig/create).
4. Set the environment variable `CLIENT_ID` to your client ID.
5. Deploy and check your domains to use the API! 🎉

</details>

## 🤝 Contribute
Contributions are welcome! Feel free to open issues or submit pull requests!

## 📜 License
MIT © [IchBinLeoon](https://github.com/IchBinLeoon/mal-readme-stats/blob/main/LICENSE)
