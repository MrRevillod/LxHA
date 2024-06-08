
import { api } from "../lib/axios"
import { create } from "zustand"
import { useHttpStore } from "./HttpStore"
import { FROM_KIND, Message, MessageData } from "../lib/types"

export interface MessageStore {
    data: Message[],
    dataSplice: Message[],
    filteredData: Message[],
    itemsPerPage: number,
    setSplicedData: (page: number) => void,
    filterBySearch: (search: string) => void,
    filterByFrom: (filter: FROM_KIND) => void
}

export interface MessageActions {
    getMessages: () => void,
    fromAdminMessage: (userId: string, message: MessageData) => Promise<number> | any,
    fromUserMessage: (userId: string, message: MessageData) => Promise<number> | any,
    deleteMessage: (id: string) => Promise<void> | any,
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

        try {

            const res = await api.get("/dashboard/messages")
            set({ data: res.data.messages, filteredData: res.data.messages })

            get().filterByFrom(FROM_KIND.USER)

        } catch (error) {

            console.log(error)
            throw new Error("Failed on fetching messages")
        }
    },

    fromAdminMessage: async (userId: string, message: MessageData) => {

        const { setResponse, setIsLoading } = useHttpStore.getState()

        try {

            setIsLoading(true)

            const res = await api.post(`/dashboard/messages/from-admin/${userId}`, message)
            setResponse(res.status, "The message was sent", res.data, true)

            return res.status

        } catch (e: any) {
            setResponse(e.response.status, e.response.data.message, e.response.data, true)
            return e.response.status

        } finally {
            setIsLoading(false)
        }
    },

    fromUserMessage: async (userId: string, message: MessageData) => {

        const { setResponse, setIsLoading } = useHttpStore.getState()

        try {

            const res = await api.post(`/dashboard/messages/from-user/${userId}`, message)
            setResponse(res.status, `The message was sent. 
                The response will be sent to your email as soon as possible.`,
                res.data, true
            )

            return res.status

        } catch (e: any) {
            setResponse(e.response.status, e.response.data.message, e.response.data, true)
            return e.response.status

        } finally {
            setIsLoading(false)
        }
    },

    deleteMessage: async (id: string) => {

        const { setResponse, setIsLoading } = useHttpStore.getState()

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
            message.from.name.toLowerCase().includes(search.toLowerCase()) ||
            message.body.toLowerCase().includes(search.toLowerCase())
        )

        set({ filteredData, dataSplice: filteredData.slice(0, get().itemsPerPage) })
    },

    filterByFrom: (filter: FROM_KIND) => {

        const filteredData = get().data.filter((message: Message) =>
            message.from_kind === filter
        )

        console.log(filter)
        console.log(get().data)
        console.log(get().filteredData)

        set({ filteredData, dataSplice: filteredData.slice(0, get().itemsPerPage) })
    }

}))
