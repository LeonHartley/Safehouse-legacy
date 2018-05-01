const state = {
  contacts: {},
  activeContact: null
}

const mutations = {
  setContacts (state, payload) {
    state.contacts = {}

    for (var i = 0; i < payload.length; i++) {
      let contact = payload[i]

      if (i === 0) {
        state.activeContact = contact
      }

      state.contacts[contact.id] = contact
    }
  },

  setActiveContact (state, payload) {
    state.activeContact = payload.activeContact
  },

  updateContactStatus (state, payload) {
    state.contacts[payload.id].status = payload.status.toLowerCase()

    if (state.activeContact.id === payload.id) {
      state.activeContact.status = payload.status.toLowerCase()
    }
  }
}

export default {
  state,
  mutations
}
