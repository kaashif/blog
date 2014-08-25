import Muon

cfg = Config { siteName = "/dev/kaashif"
             , siteTagline = "Programming, software freedom and Unix"
             , author = "Kaashif Hymabaccus"
             , url = "http://kaashif.co.uk"
             , remoteServer = "server"
             , remoteUser = "www"
             , remoteDir = "/var/www/htdocs/blog/"
             }

main = muonWith cfg $ do
    from "posts" $ do
        compile PostCompiler
        route DateRoute

    from "static" $ do
        compile IdCompiler
        route StaticRoute

    from "pages" $ do
        compile MarkdownCompiler
        route BaseRoute

    to "/rss.xml" $ do
        generateWith FeedGenerator

    to "/archive/index.html" $ do
        generateWith ArchiveGenerator

    to "/index.html" $ do
        generateWith HomeGenerator
