
import { api } from "../lib/axios"
import { users } from "../lib/data"
import { create } from "zustand"
import { useHttpStore } from "./HttpStore"
import { RegisterData, User } from "../lib/types"
import { toast } from "sonner"

export interface UserStore {
    data: User[],
    dataSplice: User[],
    filteredData: User[],
    itemsPerPage: number,
    nColumns: number,
    columns: string[],
    setSplicedData: (page: number) => void,
    filterBySearch: (search: string) => void
}

export interface UserActions {
    getUsers: () => Promise<void>,
    createUser: (user: RegisterData) => Promise<void>,
    deleteUser: (id: string) => Promise<void>,
    updateUser: (id: string, fields: any) => Promise<void>
}

const { setIsLoading, setResponse } = useHttpStore.getState()

export const useUserStore = create<UserStore & UserActions>((set, get) => ({

    data: [],
    dataSplice: [],
    filteredData: [],
    itemsPerPage: 10,

    nColumns: 7,
    columns: ["ID", "NAME", "USERNAME", "EMAIL", "ROLE", "INSTANCES", "ACTIONS"],

    setSplicedData: (pageNumber: number) => {

        const startIndex = (pageNumber - 1) * get().itemsPerPage
        const endIndex = Math.min(startIndex + get().itemsPerPage, get().filteredData.length)

        set({ dataSplice: get().filteredData.slice(startIndex, endIndex) })
    },

    getUsers: async () => {

        const fetchedMessages = users
        const dataSplice = fetchedMessages.slice(0, get().itemsPerPage);

        set({ data: fetchedMessages, filteredData: fetchedMessages, dataSplice });
    },

    createUser: async (user: RegisterData) => {

        try {

            setIsLoading(true)
            const res = await api.post("/dashboard/users", user)
            setResponse(res.status, res.data.message, res.data, true)

        } catch (error: any) {

            setResponse(error.response.status, error.response.data.message, error.response.data, true)
            console.log(error)

        } finally {
            setIsLoading(false)
        }
    },

    deleteUser: async (id: string) => {

        const { setResponse, setIsLoading } = useHttpStore()

        try {

            setIsLoading(true)
            const res = await api.delete(`/dashboard/users/${id}`)
            setResponse(res.status, res.data.message, res.data, true)

        } catch (error: any) {
            setResponse(error.response.status, error.response.data.message, error.response.data, true)
        } finally {
            setIsLoading(false)
        }
    },

    updateUser: async (id: string, fields: any) => {

        try {

            setIsLoading(true)
            const res = await api.patch(`/dashboard/users/${id}`, fields)
            setResponse(res.status, res.data.message, res.data, true)

        } catch (error: any) {
            setResponse(error.response.status, error.response.data.message, error.response.data, true)

        } finally {
            setIsLoading(false)
        }
    },

    filterBySearch: (search: string) => {

        const filteredData = get().data.filter((user: User) =>
            user.name.toLowerCase().includes(search.toLowerCase()) ||
            user.username.toLowerCase().includes(search.toLowerCase()) ||
            user.role.toLowerCase().includes(search.toLowerCase()) ||
            user.email.toLowerCase().includes(search.toLowerCase())
        )

        set({ filteredData, dataSplice: filteredData.slice(0, get().itemsPerPage) })
    }

}))
