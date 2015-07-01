{-# LANGUAGE OverloadedStrings #-}
import Data.Monoid (mappend)
import Data.List.Utils (replace)
import Data.Char (isAlphaNum, toLower)
import Text.Pandoc.Options
import Text.Pandoc.Definition
import qualified Data.Set as S
import qualified Data.Map as M
import Data.List (intersperse)
import Data.Maybe (fromJust)
import Hakyll
import Sitemap
import Text.Blaze.Html
import Text.Blaze
import qualified Text.Blaze.Html5 as H
import qualified Text.Blaze.Html5.Attributes as A
import Text.Blaze.Html.Renderer.String
import System.IO.Unsafe
import System.Process
    

-- Some stuff for highlighting with Pygments
pygTrans :: Block -> Block
pygTrans (CodeBlock (cls, [lang], _) code) =
    let composed = renderHtml $ preEscapedToHtml $ replace "</div>" "" $ replace "<div class=\"highlight\">" "" (runPygment lang code)
    in RawBlock "html" composed
pygTrans x = x

-- This is actually pure, but I cannot prove it to the compiler,
-- so unsafePerformIO _is_ appropriate.
runPygment :: String -> String -> String
runPygment lang txt = unsafePerformIO $ do
   readProcess "pygmentize" ["-l", lang, "-f", "html"] txt

pygmentize :: Pandoc -> Pandoc
pygmentize (Pandoc meta bs) = Pandoc meta (map pygTrans bs)
                
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
    in pandocCompilerWithTransform readerOptions writerOptions pygmentize

pageRoute :: Routes
pageRoute = customRoute $ replace ".markdown" "/index.html" . toFilePath

postRoute :: Routes
postRoute = metadataRoute $ \m -> constRoute $ concat $ intersperse "/" [ dateify (fromJust $ M.lookup "date" m)
                                                                        , titleify (fromJust $ M.lookup "title" m)
                                                                        , "index.html"  
                                                                        ]
titleify :: String -> String
titleify = map toLower . filter (\c -> or [isAlphaNum c, c =='-']) . replace " " "-"

dateify :: String -> String
dateify = replace "-" "/"

myFeedConfiguration = FeedConfiguration
    { feedTitle       = "/dev/kaashif"
    , feedDescription = "Programming, software freedom and Unix"
    , feedAuthorName  = "Kaashif Hymabaccus"
    , feedAuthorEmail = "kaashif@kaashif.co.uk"
    , feedRoot        = "http://www.kaashif.co.uk/"
    }

sitemapConfig :: SitemapConfiguration
sitemapConfig = def
    { sitemapBase     = "http://www.kaashif.co.uk/"
    }

config = defaultConfiguration {
           deployCommand = "cd _site; git init; git --git-dir=./.git add .; git --git-dir=./.git commit -m 'rebuilt'; git --git-dir=./.git push -f ssh://55193ee3fcf9334054000012@blog-kaashif.rhcloud.com/~/git/blog.git/ master"
         }

main :: IO ()
main = hakyllWith config $ do
    match "static/**" $ do
        route   idRoute
        compile copyFileCompiler

    match "css/*" $ do
        route   idRoute
        compile compressCssCompiler

    match (fromList ["about.markdown", "contact.markdown"]) $ do
        route   pageRoute
        compile $ myPandocCompiler
          >>= loadAndApplyTemplate "templates/default.html" defaultContext
          >>= relativizeUrls

    match "posts/*" $ do
        route   postRoute
        compile $ myPandocCompiler
          >>= saveSnapshot "content"   
          >>= loadAndApplyTemplate "templates/post.html"    postCtx
          >>= loadAndApplyTemplate "templates/default.html" postCtx
          >>= relativizeUrls
              
    create ["atom.xml"] $ do
      route idRoute
      compile $ do
        let feedCtx = postCtx `mappend` bodyField "description"
        posts <- fmap (take 10) . recentFirst =<<
            loadAllSnapshots "posts/*" "content"
        renderAtom myFeedConfiguration feedCtx posts

    create ["rss.xml"] $ do
        route idRoute
        compile $ do
          let feedCtx = postCtx `mappend` bodyField "description"
          posts <- fmap (take 10) . recentFirst =<<
              loadAllSnapshots "posts/*" "content"
          renderRss myFeedConfiguration feedCtx posts

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

    create ["sitemap.xml"] $ do
        route idRoute
        compile $ generateSitemap sitemapConfig

    match "index.html" $ do
        route idRoute
        compile $ do
            posts <- loadAllSnapshots "posts/*" "content" >>= recentFirst
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
    (field "extract" $ return.unlines.take 15.lines.itemBody) `mappend`
--  (field "extract" $ return.itemBody) `mappend`
    dateField "date" "%Y-%m-%d" `mappend`
    defaultContext
