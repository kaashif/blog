{-# LANGUAGE OverloadedStrings #-}
import Data.Monoid (mappend)
import Data.List.Utils (replace)
import Text.Pandoc.Options
import qualified Data.Set as S
import Hakyll

myPandocCompiler =
    let mathExtensions = [Ext_tex_math_dollars]
        defaultExtensions = writerExtensions defaultHakyllWriterOptions
        newExtensions = foldr S.insert defaultExtensions mathExtensions
        readerOptions = defaultHakyllReaderOptions {
                          readerSmart = False
                        }
        writerOptions = defaultHakyllWriterOptions {
                          writerExtensions = newExtensions,
                          writerHTMLMathMethod = MathJax ""
                        }
    in pandocCompilerWith readerOptions writerOptions

pageRoute :: Routes
pageRoute = customRoute $ replace ".markdown" "/index.html" . toFilePath

main :: IO ()
main = hakyll $ let pandocCompiler = myPandocCompiler in do
    match "static/*" $ do
        route   idRoute
        compile copyFileCompiler

    match "css/*" $ do
        route   idRoute
        compile compressCssCompiler

    match (fromList ["about.markdown", "contact.markdown"]) $ do
        route   pageRoute
        compile $ pandocCompiler
            >>= loadAndApplyTemplate "templates/default.html" defaultContext
            >>= relativizeUrls

    match "posts/*" $ do
        route $ setExtension "html"
        compile $ pandocCompiler
            >>= loadAndApplyTemplate "templates/post.html"    postCtx
            >>= loadAndApplyTemplate "templates/default.html" postCtx
            >>= relativizeUrls

    create ["archive/index.html"] $ do
        route idRoute
        compile $ do
            posts <- loadAll "posts/*" >>= recentFirst
            let archiveCtx =
                    listField "posts" postCtx (return posts) `mappend`
                    constField "title" "Archives"            `mappend`
                    defaultContext

            makeItem ""
                >>= loadAndApplyTemplate "templates/archive.html" archiveCtx
                >>= loadAndApplyTemplate "templates/default.html" archiveCtx
                >>= relativizeUrls


    match "index.html" $ do
        route idRoute
        compile $ do
            posts <- loadAll "posts/*" >>= recentFirst
            let indexCtx =
                    listField "posts" postCtx (return posts) `mappend`
                    constField "title" "Home"                `mappend`
                    defaultContext

            getResourceBody
                >>= applyAsTemplate indexCtx
                >>= loadAndApplyTemplate "templates/default.html" indexCtx
                >>= relativizeUrls

    match "templates/*" $ compile templateCompiler

postCtx :: Context String
postCtx =
    -- "extract" field contains first 25 lines (80 chars each)
    (field "extract" $ return . take 2000 . unlines . drop 31 . lines . itemBody) `mappend`
    dateField "date" "%Y-%m-%d" `mappend`
    defaultContext
