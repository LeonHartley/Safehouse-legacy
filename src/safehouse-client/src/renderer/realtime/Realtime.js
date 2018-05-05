import Auth from '../api/auth/Auth'
import Message from './Message'
import Store from '../store'
import cryptico from 'cryptico'

var connection = {
  socket: null,
  server: null,
  key: {}
}

var handlers = {
  '2': (msg) => {
    Store.commit('updateContactStatus', JSON.parse(msg.payload))
  },
  '3': (msg) => {
    var payload = JSON.parse(msg.payload)

    payload.message = JSON.parse(cryptico.decrypt(payload.message, connection.key).plaintext)

    Store.commit('newChatMessage', payload)
  }
}

var connectionReady = (event) => {
  console.log('Safehouse-Realtime - Ready for messages')

  let token = Auth.getAuthToken()
  connection.key = cryptico.generateRSAKey(token, 1024)

  sendMessage(new Message(1, JSON.stringify({
    token: token,
    key: cryptico.publicKeyString(connection.key)
  })))

  sendMessage(new Message(2))
}

var handleMessage = (event) => {
  getBuffer(event.data, (buffer) => {
    var msg = Message.decode(buffer)
    console.log('msg type: ' + msg.type + ', payload: ' + msg.payload)

    handlers[msg.type](msg)
  })
}

var sendMessage = (message) => {
  if (connection.socket.readyState === 1) {
    // todo: expandable buffer..
    connection.socket.send(message.encode(new ArrayBuffer(1024)))
  }
}

var sendChatMessage = (message) => {
  message.message = cryptico.encrypt(
    JSON.stringify(message.message),
    Store.getters.publicKeyForContact(message.user_id)
  ).cipher

  sendMessage(new Message(3, JSON.stringify(message)))
}

var disconnect = () => {
  connection.socket.close()
}

function getBuffer (blob, consumer) {
  var fileReader = new FileReader()

  fileReader.onload = function () {
    consumer(this.result)
  }

  fileReader.readAsArrayBuffer(blob)
}

export default {
  connect (server) {
    connection.server = server
    connection.socket = new WebSocket('ws://' + server.host + ':' + server.port)

    connection.socket.onmessage = handleMessage
    connection.socket.onopen = (e) => {
      connectionReady(e)
      server.ready()
    }
  },

  sendChatMessage: sendChatMessage,
  send: sendMessage,

  disconnect: disconnect,
  Message: Message
}
