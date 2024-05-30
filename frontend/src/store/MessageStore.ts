
import { api } from "../lib/axios"
import { create } from "zustand"
import { messages } from "../lib/data"
import { useHttpStore } from "./HttpStore"
import { Message, MessageData } from "../lib/types"

export interface MessageStore {
    data: Message[],
    dataSplice: Message[],
    filteredData: Message[],
    itemsPerPage: number,
    setSplicedData: (page: number) => void,
    filterBySearch: (search: string) => void
}

export interface MessageActions {
    getMessages: () => void,
    sendSupportMessage: (message: MessageData) => void,
    deleteMessage: (id: string) => void,
}

export const useMessageStore = create<MessageStore & MessageActions>((set, get) => ({

    data: [],
    dataSplice: [],
    filteredData: [],
    itemsPerPage: 7,

    setSplicedData: (pageNumber: number) => {

        const startIndex = (pageNumber - 1) * get().itemsPerPage
        const endIndex = Math.min(startIndex + get().itemsPerPage, get().filteredData.length)

        set({ dataSplice: get().filteredData.slice(startIndex, endIndex) })
    },

    getMessages: async () => {

        const fetchedMessages = messages

        set({ data: fetchedMessages, filteredData: fetchedMessages })
        set({ dataSplice: fetchedMessages.slice(0, get().itemsPerPage) })
    },

    sendSupportMessage: async (message: MessageData) => {

        const { setResponse, setIsLoading } = useHttpStore()

        try {

            const res = await api.post("/dashboard/messages", message)

            setResponse(res.status,
                `The message was sent. 
                The response will be sent to your email as soon as possible.`,
                res.data, true
            )

        } catch (e: any) {
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally {
            setIsLoading(false)
        }
    },

    deleteMessage: async (id: string) => {

        const { setResponse, setIsLoading } = useHttpStore()

        try {

            await api.delete(`/dashboard/messages/${id}`)

            setResponse(200, "Message deleted successfully", {}, true)

        } catch (e: any) {
            setResponse(e.response.status, e.response.data.message, e.response.data, true)

        } finally {
            setIsLoading(false)
        }
    },

    filterBySearch: (search: string) => {

        const filteredData = get().data.filter((message: Message) =>
            message.from.toLowerCase().includes(search.toLowerCase()) ||
            message.message.toLowerCase().includes(search.toLowerCase())
        )

        set({ filteredData, dataSplice: filteredData.slice(0, get().itemsPerPage) })
    }

}))
