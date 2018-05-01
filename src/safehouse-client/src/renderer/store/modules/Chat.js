const state = {
  chats: {},
  activeChat: null
}

const mutations = {
  newChatMessage (state, payload) {
    if (state.chats[payload.sender] === undefined) {
      state.chats[payload.sender] = {
        user_id: payload.sender,
        messages: []
      }
    }

    var msg = {
      from: payload.sender,
      txt: payload.message
    }

    state.chats[payload.sender].messages.push(msg)

    if (state.activeChat.user_id === payload.sender) {
      state.activeChat.messages.push(msg)
    }
  },

  startChat (state, payload) {
    if (state.chats[payload] === undefined) {
      state.chats[payload] = {
        user_id: payload,
        messages: []
      }
    }

    state.activeChat = state.chats[payload]
  }
}

export default {
  state,
  mutations
}
