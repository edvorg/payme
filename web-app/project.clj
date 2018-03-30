(defproject payme "0.1.0-SNAPSHOT"
  :description "Invoice your client in one click"
  :url "https://github.com/edvorg/payme"
  :license {:name "Eclipse Public License"
            :url "http://www.eclipse.org/legal/epl-v10.html"}

  :min-lein-version "2.7.1"

  :dependencies [[org.clojure/clojure "1.9.0"]
                 [org.clojure/clojurescript "1.10.64"]
                 [org.clojure/core.async  "0.4.474"]
                 [reagent "0.7.0"]
                 [secretary "1.2.3"]
                 [venantius/accountant "0.2.4" :exclusions [org.clojure/tools.reader]]
                 [cljs-http "0.1.44"]
                 [rocks.clj/configuron "0.1.1-SNAPSHOT"]
                 [reagent-utils "0.3.1"]
                 [rocks.clj/transit "0.1.0-SNAPSHOT"]]

  :plugins [[lein-figwheel "0.5.15"]
            [lein-cljsbuild "1.1.7" :exclusions [[org.clojure/clojure]]]
            [rocks.clj/lein-give-me-my-css "0.1.0-SNAPSHOT"]]

  :source-paths ["src"]

  :cljsbuild {:builds
              [{:id "dev"
                :source-paths ["src"]
                :figwheel {:on-jsload "payme.core/on-js-reload"
                           :open-urls ["http://localhost:3449/index.html"]}
                :compiler {:main payme.core
                           :asset-path "js/compiled/out"
                           :output-to "resources/public/js/compiled/payme.js"
                           :output-dir "resources/public/js/compiled/out"
                           :source-map-timestamp true
                           :preloads [devtools.preload]
                           :optimizations :none
                           :foreign-libs [{:file "resources/public/node_modules/react-recaptcha/dist/react-recaptcha.js"
                                           :file-min "resources/public/node_modules/react-recaptcha/dist/react-recaptcha.js"
                                           :provides ["cljsjs.react-recaptcha"]}]}}
               {:id "min"
                :source-paths ["src"]
                :compiler {:output-to "resources/public/js/compiled/payme.js"
                           :main payme.core
                           :optimizations :none #_:advanced
                           :pretty-print false}}]}

  :figwheel {:css-dirs ["resources/public/css"]}

  :profiles {:dev {:dependencies [[binaryage/devtools "0.9.9"]
                                  [figwheel-sidecar "0.5.15"]
                                  [com.cemerick/piggieback "0.2.2"]]
                   :source-paths ["src" "dev"]
                   :repl-options {:nrepl-middleware [cemerick.piggieback/wrap-cljs-repl]}
                   :clean-targets ^{:protect false} ["resources/public/js/compiled"
                                                     "resources/public/css"
                                                     :target-path]}})
