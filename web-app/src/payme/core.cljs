(ns payme.core
  (:require [reagent.core :as reagent :refer [atom cursor]]
            [cljs.core.async :refer [go <! timeout]]
            [cljs-http.client :as http])
  (:require-macros [cljs.core.async :refer [go]]))

(enable-console-print!)

(def app-state (atom {:messages []}))

(defn show-message [component]
  (swap! app-state update :messages conj component))

(declare message-view)
(declare email-view)
(declare client-email-view)
(declare task-view)
(declare hours-view)
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
                :on-key-press (on-enter #(show-message [client-email-view]))}]])))

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
                :on-key-press (on-enter #(show-message [task-view]))}]])))

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
                :on-key-press (on-enter #(show-message [hours-view]))}]])))

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
                :on-key-press (on-enter #(show-message [company-view]))}]])))

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
                :on-key-press (on-enter #(show-message [company-address-view]))}]])))

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
                :on-key-press (on-enter #(show-message [client-company-view]))}]])))

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
                :on-key-press (on-enter #(show-message [client-company-address-view]))}]])))

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
                :on-key-press (on-enter #(show-message [terms-view]))}]])))

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
                   :on-key-press (on-enter #(show-message [ready-view])
                                           #(swap! terms str "\n"))}]])))

(defn send-invoice []
  (show-message [message-view "Sending your invoice..."])
  (go
    (let [params   (:params @app-state)
          response (<! (http/post "localhost:3000" #_"http://payme.rust.cafe"
                                  {:json-params params}))]
      (show-message [message-view (str "Params " (pr-str params)
                                       "\nResult " (pr-str response))]))))

(defn ready-view []
  (let []
    (fn []
      [:div.card.ready
       [:label "All set. Ready to send invoice? Hit ENTER :)"]
       [:input {:type         :string
                :value        ""
                :on-change    (fn [_])
                :auto-focus   true
                :on-key-press (on-enter send-invoice)}]])))

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
    (show-message [message-view "Answer a few questions and I'll remember it for the next time you come back"])
    (<! (timeout 1500))
    (show-message [email-view])))
