(ns payme.core
  (:require [reagent.core :as reagent :refer [atom cursor]]
            [cljs.core.async :refer [go <! timeout]]
            [cljs-http.client :as http]
            [clojure.string :as s]
            [cljsjs.react-recaptcha]
            [reagent.cookies :as cookies]
            [goog.crypt.base64 :as b64]
            [goog.crypt :as cr]
            [rocks.clj.transit.core :as transit]
            [cljsjs.react-datetime])
  (:require-macros [cljs.core.async :refer [go]]))

(def datetime (reagent/adapt-react-class js/Datetime))

(enable-console-print!)

(def app-state (atom {:messages []
                      :params (or (when-let [d (cookies/get "payme_invoice")]
                                    (transit/from-transit (cr/utf8ByteArrayToString (b64/decodeStringToByteArray d true))))
                                  {})}))

(defn write-cookie []
  (cookies/set! "payme_invoice" (b64/encodeByteArray (cr/stringToUtf8ByteArray (transit/to-transit (:params @app-state))) true)))

(defn focus-element [id]
  (.focus (.getElementById js/document id)))

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
        "Enter" (do
                  (go
                    (f))
                  (.preventDefault e)
                  e)
        e))))

(defn message-view [message]
  [:div.card.message
   message])

(defn email-view []
  (let [email (cursor app-state [:params :email])]
    (fn []
      [:span.card.email
       [:input {:type :string
                :id "email-view"
                :value @email
                :placeholder "Your email"
                :on-change (fn [e]
                             (reset! email (.. e -target -value))
                             (write-cookie))
                :on-key-press (on-enter #(focus-element "client-email-view"))}]])))

(defn client-email-view []
  (let [client-email (cursor app-state [:params :client-email])]
    (fn []
      [:span.card.client-email
       [:input {:type :string
                :id "client-email-view"
                :value @client-email
                :placeholder "Client's email"
                :on-change (fn [e]
                             (reset! client-email (.. e -target -value))
                             (write-cookie))
                :on-key-press (on-enter #(focus-element "send-button-view"))}]])))

(defn task-view []
  (let [task (cursor app-state [:params :task])]
    (fn []
      [:div.card.task
       [:input {:type :string
                :id "task-view"
                :value @task
                :placeholder "What did you work on?"
                :on-change (fn [e]
                             (reset! task (.. e -target -value))
                             (write-cookie))
                :on-key-press (on-enter #(focus-element "hours-view"))}]])))

(defn hours-view []
  (let [hours (cursor app-state [:params :hours])]
    (fn []
      [:div.card.hours
       [:input {:type :string
                :id "hours-view"
                :value @hours
                :placeholder "Number of units"
                :on-change (fn [e]
                             (reset! hours (.. e -target -value))
                             (write-cookie))
                :on-key-press (on-enter #(focus-element "rate-view"))}]])))

(defn rate-view []
  (let [rate (cursor app-state [:params :rate])]
    (fn []
      [:div.card.rate
       [:input {:type :string
                :id "rate-view"
                :value @rate
                :placeholder "Rate per unit"
                :on-change (fn [e]
                             (reset! rate (.. e -target -value))
                             (write-cookie))
                :on-key-press (on-enter #(focus-element "number-view"))}]])))

(defn company-view []
  (let [company (cursor app-state [:params :company])]
    (fn []
      [:div.card.company
       [:input {:type :string
                :id "company-view"
                :value @company
                :placeholder "Your company name"
                :on-change (fn [e]
                             (reset! company (.. e -target -value))
                             (write-cookie))
                :on-key-press (on-enter #(focus-element "company-address-view"))
                :auto-focus true}]])))

(defn company-address-view []
  (let [company-address (cursor app-state [:params :company-address])]
    (fn []
      [:div.card.company-address
       [:input {:type :string
                :id "company-address-view"
                :value @company-address
                :placeholder "Your company address"
                :on-change (fn [e]
                             (reset! company-address (.. e -target -value))
                             (write-cookie))
                :on-key-press (on-enter #(focus-element "client-company-view"))}]])))

(defn client-company-view []
  (let [client-company (cursor app-state [:params :client-company])]
    (fn []
      [:div.card.client-company
       [:input {:type :string
                :id "client-company-view"
                :value @client-company
                :placeholder "Client company name"
                :on-change (fn [e]
                             (reset! client-company (.. e -target -value))
                             (write-cookie))
                :on-key-press (on-enter #(focus-element "client-company-address-view"))}]])))

(defn client-company-address-view []
  (let [client-company-address (cursor app-state [:params :client-company-address])]
    (fn []
      [:div.card.client-company-address
       [:input {:type :string
                :id "client-company-address-view"
                :value @client-company-address
                :placeholder "Client company address"
                :on-change (fn [e]
                             (reset! client-company-address (.. e -target -value))
                             (write-cookie))
                :on-key-press (on-enter #(focus-element "task-view"))}]])))

(defn terms-view []
  (let [terms (cursor app-state [:params :terms])]
    (fn []
      [:div.card.terms
       [:textarea {:type :string
                   :id "terms-view"
                   :value (s/replace (str @terms) "<ENTER>" "\n")
                   :on-change (fn [e]
                                (reset! terms (s/replace (str (.. e -target -value)) "\n" "<ENTER>"))
                                (write-cookie))
                   ;; :on-key-press (on-enter #(focus-element "number-view"))
                   }]])))

(defn number-view []
  (let [number (cursor app-state [:params :number])]
    (fn []
      [:input {:type :string
               :id "number-view"
               :value @number
               :on-change (fn [e]
                            (reset! number (.. e -target -value))
                            (write-cookie))
               :on-key-press (on-enter #(focus-element "email-view"))}])))

(defn send-invoice [verify message g-recaptcha-response]
  (go
    (reset! message "Sending your invoice...")
    (reset! verify false)
    (let [date             (:date @app-state)
          params           (->> (:params @app-state)
                                (map (fn [[k v]]
                                       (let [k (-> k
                                                   name
                                                   (s/replace "-" "_")
                                                   keyword)]
                                         [k v])))
                                (into {}))
          params           (cond-> params
                             date (assoc :date date))
          {:keys [success
                  body]
           :as   response} (<! (http/post "/invoice"
                                          {:json-params  params
                                           :query-params {:g-recaptcha-response g-recaptcha-response}}))]
      (<! (timeout 500))
      (if success
        (reset! message "Done! Your invoice has been sent")
        (reset! message (str "\nResult: " body)))
      (<! (timeout 2000))
      (reset! message nil))))

(defn ready-view []
  (let [verify (cursor app-state [:verify])
        message (atom nil)]
    (fn []
      [:div.card.ready
       (when @message
         [:div @message])
       (when @verify
         [:> js/ReactRecaptcha {:sitekey "6Lcmw00UAAAAAOOKJDoeVNEsVuJFJ6ka3dSbGaIV"
                                :verifyCallback (fn [g-recaptcha-response]
                                                  (send-invoice verify message (js->clj g-recaptcha-response)))}])])))

(defn send-button-view []
  (let [verify (cursor app-state [:verify])]
    (fn []
      [:button.send-button {:id "send-button-view"
                            :on-click #(reset! verify true)}
       "Send"])))

(defn send-view []
  (let [verify (cursor app-state [:verify])]
    (fn []
      (if-not @verify
        [:div
         [email-view]
         [client-email-view]
         [send-button-view]]
        [:div]))))

(defn hello-world []
  (let [hours (cursor app-state [:params :hours])
        rate (cursor app-state [:params :rate])
        date (cursor app-state [:date])]
    (fn []
      (let [total (* (.parseInt js/window @hours)
                     (.parseInt js/window @rate))
            total (if (js/isNaN total)
                    0
                    total)]
        [:div
         [:form
          (for [[i component] (map-indexed vector (:messages @app-state))]
            (with-meta component {:key i}))]
         [:div.invoice {:style {:max-width "1024px"}}
          [:h1#title "INVOICE"]
          [:div#number
           "#"
           [number-view]]
          [:div#date
           [:span "Date: "]
           [datetime {:default-value (js/Date.)
                      :time-format false
                      :on-change #(reset! date (.format % "MMMM DD, YYYY"))
                      :style {:display :inline}
                      :date-format "MMMM DD, YYYY"
                      :close-on-select true}]]
          [:div.balance
           [:span "Balance Due: "]
           [:span.value total]]
          [:div#company
           [company-view]
           [company-address-view]]
          [:div "Bill to:"]
          [:div#client_company
           [client-company-view]
           [client-company-address-view]]
          [:table#items
           [:thead
            [:tr
             [:th [:div "Item"]]
             [:th [:div "Quantity"]]
             [:th [:div "Rate"]]
             [:th [:div "Amount"]]]]
           [:tbody
            [:tr
             [:td
              [task-view]]
             [:td
              [hours-view]]
             [:td
              [rate-view]]
             [:td
              [:span.value total]]]]]
          [:div.balance
           [:span "Subtotal: "]
           [:span.value total]]
          [:div.balance
           [:span "Total: "]
           [:span.value total]]
          [:div "Terms:"]
          [terms-view]]
         [:div.send
          [send-view]
          [ready-view]]]))))

(reagent/render-component [hello-world]
                          (. js/document (getElementById "app")))
