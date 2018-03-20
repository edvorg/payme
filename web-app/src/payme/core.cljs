(ns payme.core
  (:require [reagent.core :as reagent :refer [atom cursor]]
            [cljs.core.async :refer [go <! timeout]]
            [cljs-http.client :as http]
            [clojure.string :as s]
            [cljsjs.react-recaptcha]
            [reagent.cookies :as cookies])
  (:require-macros [cljs.core.async :refer [go]]))

(enable-console-print!)

(def app-state (atom {:messages []
                      :params (cookies/get "data" {})}))

(defn show-message [component]
  (swap! app-state update :messages conj component))

(declare message-view)
(declare email-view)
(declare client-email-view)
(declare task-view)
(declare hours-view)
(declare rate-view)
(declare company-view)
(declare company-address-view)
(declare client-company-view)
(declare client-company-address-view)
(declare terms-view)
(declare ready-view)

(defn on-enter [f & [g]]
  (let [g (or g identity)]
    (fn [e]
      (case (.-key e)
        "Enter" (if (.-ctrlKey e)
                  (do
                    (go
                      (g))
                    e)
                  (do
                    (go
                      (f))
                    (.preventDefault e)
                    e))
        e))))

(defn message-view [message]
  [:div.card.message
   message])

(defn email-view []
  (let [email (cursor app-state [:params :email])]
    (fn []
      [:div.card.email
       [:label "What is your email?"]
       [:input {:type :string
                :value @email
                :on-change (fn [e]
                             (reset! email (.. e -target -value)))
                :auto-focus true
                :on-key-press (on-enter #(do
                                           (show-message [client-email-view])
                                           (cookies/set! "data" (:params @app-state) :raw? false)))}]])))

(defn client-email-view []
  (let [client-email (cursor app-state [:params :client-email])]
    (fn []
      [:div.card.client-email
       [:label "What is your client's email?"]
       [:input {:type :string
                :value @client-email
                :on-change (fn [e]
                             (reset! client-email (.. e -target -value)))
                :auto-focus true
                :on-key-press (on-enter #(do
                                           (show-message [task-view])
                                           (cookies/set! "data" (:params @app-state) :raw? false)))}]])))

(defn task-view []
  (let [task (cursor app-state [:params :task])]
    (fn []
      [:div.card.task
       [:label "What did you work on this month?"]
       [:input {:type :string
                :value @task
                :on-change (fn [e]
                             (reset! task (.. e -target -value)))
                :auto-focus true
                :on-key-press (on-enter #(do
                                           (show-message [hours-view])
                                           (cookies/set! "data" (:params @app-state) :raw? false)))}]])))

(defn hours-view []
  (let [hours (cursor app-state [:params :hours])]
    (fn []
      [:div.card.hours
       [:label "How many hours did you spend?"]
       [:input {:type :string
                :value @hours
                :on-change (fn [e]
                             (reset! hours (.. e -target -value)))
                :auto-focus true
                :on-key-press (on-enter #(do
                                           (show-message [rate-view])
                                           (cookies/set! "data" (:params @app-state) :raw? false)))}]])))

(defn rate-view []
  (let [rate (cursor app-state [:params :rate])]
    (fn []
      [:div.card.rate
       [:label "What is your hourly rate?"]
       [:input {:type :string
                :value @rate
                :on-change (fn [e]
                             (reset! rate (.. e -target -value)))
                :auto-focus true
                :on-key-press (on-enter #(do
                                           (show-message [company-view])
                                           (cookies/set! "data" (:params @app-state) :raw? false)))}]])))

(defn company-view []
  (let [company (cursor app-state [:params :company])]
    (fn []
      [:div.card.company
       [:label "What is your company's name?"]
       [:input {:type :string
                :value @company
                :on-change (fn [e]
                             (reset! company (.. e -target -value)))
                :auto-focus true
                :on-key-press (on-enter #(do
                                           (show-message [company-address-view])
                                           (cookies/set! "data" (:params @app-state) :raw? false)))}]])))

(defn company-address-view []
  (let [company-address (cursor app-state [:params :company-address])]
    (fn []
      [:div.card.company-address
       [:label "What is your company's address?"]
       [:input {:type :string
                :value @company-address
                :on-change (fn [e]
                             (reset! company-address (.. e -target -value)))
                :auto-focus true
                :on-key-press (on-enter #(do
                                           (show-message [client-company-view])
                                           (cookies/set! "data" (:params @app-state) :raw? false)))}]])))

(defn client-company-view []
  (let [client-company (cursor app-state [:params :client-company])]
    (fn []
      [:div.card.client-company
       [:label "What is your client's company name?"]
       [:input {:type :string
                :value @client-company
                :on-change (fn [e]
                             (reset! client-company (.. e -target -value)))
                :auto-focus true
                :on-key-press (on-enter #(do
                                           (show-message [client-company-address-view])
                                           (cookies/set! "data" (:params @app-state) :raw? false)))}]])))

(defn client-company-address-view []
  (let [client-company-address (cursor app-state [:params :client-company-address])]
    (fn []
      [:div.card.client-company-address
       [:label "What is your client's company address?"]
       [:input {:type :string
                :value @client-company-address
                :on-change (fn [e]
                             (reset! client-company-address (.. e -target -value)))
                :auto-focus true
                :on-key-press (on-enter #(do
                                           (show-message [terms-view])
                                           (cookies/set! "data" (:params @app-state) :raw? false)))}]])))

(defn terms-view []
  (let [terms (cursor app-state [:params :terms])]
    (fn []
      [:div.card.terms
       [:label "Any notes for terms and agreements section?"]
       [:label "(Ctrl-Enter for line break)"]
       [:textarea {:type :string
                   :value @terms
                   :on-change (fn [e]
                                (reset! terms (.. e -target -value)))
                   :auto-focus true
                   :on-key-press (on-enter #(do
                                              (show-message [ready-view])
                                              (cookies/set! "data" (:params @app-state) :raw? false))
                                           #(swap! terms str "\n"))}]])))

(defn send-invoice [g-recaptcha-response]
  (show-message [message-view "Sending your invoice..."])
  (go
    (let [params           (->> (:params @app-state)
                                (map (fn [[k v]]
                                       (let [k (-> k
                                                   name
                                                   (s/replace "-" "_")
                                                   keyword)]
                                         [k v])))
                                (into {}))
          {:keys [success]
           :as   response} (<! (http/post "http://localhost:3000/invoice" #_"http://rust.cafe/invoice"
                                          {:json-params params
                                           :query-params {:g-recaptcha-response g-recaptcha-response}}))]
      (<! (timeout 500))
      (if success
        (show-message [message-view "Done! Your invoice has been sent"])
        (show-message [message-view (str "Params " (pr-str params)
                                         "\nResult " (pr-str response))])))))

(defn ready-view []
  (let [verified (atom nil)]
    (fn []
      [:div.card.ready
       [:label "All set. Ready to send invoice? Just pass captcha"]
       (when-not @verified
         [:> js/ReactRecaptcha {:sitekey "6Lcmw00UAAAAAOOKJDoeVNEsVuJFJ6ka3dSbGaIV"
                                :verifyCallback (fn [g-recaptcha-response]
                                                  (reset! verified true)
                                                  (send-invoice (js->clj g-recaptcha-response)))}])])))

(defn hello-world []
  [:div
   [:form
    (for [[i component] (map-indexed vector (:messages @app-state))]
      (with-meta component {:key i}))]])

(reagent/render-component [hello-world]
                          (. js/document (getElementById "app")))

(def init
  (go
    (show-message [message-view "Hi"])
    (<! (timeout 1500))
    (show-message [message-view "Let's build your invoice"])
    (<! (timeout 1500))
    (show-message [message-view "Answer a few questions and I'll remember your answers for the next time you come back"])
    (<! (timeout 1500))
    (show-message [email-view])))
