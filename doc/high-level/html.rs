events {
  "main" main
  "html" html
  "state" state-get
  "increment" increment
}

state #{
  #counter 0
}

let main = fn [str] args to void with async do
             let loop = fn void to void do
               wait ()
               loop ()
             end
           end

let html = fn void to str do
             "<html>
                <head></head>
                <body>
                  <script>
                    const ws = WebSocket(\"ws://localhost:7777\")
                    const state = {
                      counter: 0
                    }
                    const onClick = () => {
                      ws.send(\"increment\", (data) => { state = data})
                    }
                    ws.send(\"state\", (data) => { state = data })
                  </script>
                  <h1>This is my counter</h1>
                  <button on-click={onClick}>Counter: {{ state.counter }}</button>
                </body>
              </html>"
           end

let increment = fn void to state with state do
                  state-update fn state it to state with state do
                                  #{ #counter (+ 1 (#counter it)) }
                               end
                  state-get ()
                end

