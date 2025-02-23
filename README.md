<h1 align="center">MyAnimeList Readme Stats</h1>

<p align="center">Dynamically generated MyAnimeList stats for your profile readme</p>

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

### Activity
```
[![Activity](https://mal-readme-stats-ichbinleoons-projects.vercel.app/api/activity/[media]/[user])](https://github.com/IchBinLeoon/mal-readme-stats)
```

You can pass the query parameter `?limit=` to specify the number of entries. The minimum is 1, the maximum is 10, and the default is 5.

<details>
<summary>Show anime example</summary>

[![Anime Activity](https://mal-readme-stats-ichbinleoons-projects.vercel.app/api/activity/anime/IchBinLeoon)](https://github.com/IchBinLeoon/mal-readme-stats)

</details>

<details>
<summary>Show manga example</summary>

[![Manga Activity](https://mal-readme-stats-ichbinleoons-projects.vercel.app/api/activity/manga/IchBinLeoon)](https://github.com/IchBinLeoon/mal-readme-stats)

</details>

### Statistics
_Coming soon..._

## 🔧 Deploy your own

### Vercel
1. Register or log in to [Vercel](https://vercel.com)
2. Fork this repository and import it
3. Create a MyAnimeList API Application [here](https://myanimelist.net/apiconfig/create)
4. Set the environment variable `CLIENT_ID` to your Client ID
5. Deploy! Check your project domains to use the API 🎉

## 🤝 Contribute
Contributions are welcome! Feel free to open issues or submit pull requests!

## 📜 License
MIT © [IchBinLeoon](https://github.com/IchBinLeoon/mal-readme-stats/blob/main/LICENSE)
