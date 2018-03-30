(ns payme.core
  (:require [reagent.core :as reagent :refer [atom cursor]]
            [cljs.core.async :refer [go <! timeout]]
            [cljs-http.client :as http]
            [clojure.string :as s]
            [cljsjs.react-recaptcha]
            [reagent.cookies :as cookies]
            [goog.crypt.base64 :as b64]
            [rocks.clj.transit.core :as transit])
  (:require-macros [cljs.core.async :refer [go]]))

(enable-console-print!)

(def app-state (atom {:messages []
                      :params (or (when-let [d (cookies/get-raw "payme_invoice")]
                                    (transit/from-transit (b64/decodeString d)))
                                  {})}))

(defn write-cookie []
  (cookies/set! "payme_invoice" (b64/encodeString (transit/to-transit (:params @app-state)))
                :raw? true))

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
(declare number-view)
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
                                           (write-cookie)))}]])))

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
                                           (write-cookie)))}]])))

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
                                           (write-cookie)))}]])))

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
                                           (write-cookie)))}]])))

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
                                           (write-cookie)))}]])))

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
                                           (write-cookie)))}]])))

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
                                           (write-cookie)))}]])))

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
                                           (write-cookie)))}]])))

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
                                           (write-cookie)))}]])))

(defn terms-view []
  (let [terms (cursor app-state [:params :terms])]
    (fn []
      [:div.card.terms
       [:label "Any notes for terms and agreements section?"]
       [:label "(Ctrl-Enter for line break, empty line for paragraph)"]
       [:textarea {:type :string
                   :value (s/replace (str @terms) "<ENTER>" "\n")
                   :on-change (fn [e]
                                (reset! terms (s/replace (str (.. e -target -value)) "\n" "<ENTER>")))
                   :auto-focus true
                   :on-key-press (on-enter #(do
                                              (show-message [number-view])
                                              (write-cookie))
                                           #(swap! terms str "<ENTER>"))}]])))

(defn number-view []
  (let [number (cursor app-state [:params :number])]
    (fn []
      [:div.card.number
       [:label "What's the number of your invoice?"]
       [:input {:type :string
                :value @number
                :on-change (fn [e]
                             (reset! number (.. e -target -value)))
                :auto-focus true
                :on-key-press (on-enter #(do
                                           (show-message [ready-view])
                                           (write-cookie)))}]])))

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
          {:keys [success
                  body]
           :as   response} (<! (http/post "/invoice"
                                          {:json-params  params
                                           :query-params {:g-recaptcha-response g-recaptcha-response}}))]
      (<! (timeout 500))
      (if success
        (show-message [message-view "Done! Your invoice has been sent"])
        (show-message [message-view (str "\nResult: " body)])))))

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
