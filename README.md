<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/jawide/linkmove-rs-nwg">
    <img src="icon.ico" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">linkmove-rs-nwg</h3>

  <p align="center">
    链接移动工具，一键创建目录链接
    <br />
    基于Rust + Native Windows GUI + Win32API
    <br />
    <a href="https://github.com/jawide/linkmove-rs-nwg"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/jawide/linkmove-rs-nwg">View Demo</a>
    ·
    <a href="https://github.com/jawide/linkmove-rs-nwg/issues">Report Bug</a>
    ·
    <a href="https://github.com/jawide/linkmove-rs-nwg/issues">Request Feature</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](images/Snipaste_2024-02-24_15-07-24.png)

链接移动共分三步

1. 复制目录
2. 删除目录
3. 创建目录链接

## Feature

* ✅ 无需依赖：基于Win32API，无需引入其他依赖库
* ⚡ 动态提权：基于ShellExecuteExW，动态提权
* 🔒 安全可靠：基于SHFileOperationW，即使操作失败也可以从回收站恢复
* 🧭 路径校验：基于正则表达式校验，避免用户误操作破坏系统目录