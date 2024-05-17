
import { api } from "../lib/axios"
import { User } from "../lib/types"
import { create } from "zustand"

interface UserStore {

    user: User | null,
    setUser: (user: User | null) => void,
    createUser: (user: User) => Promise<void>,
    updateUser: (id: string, updateFields: User) => Promise<void>,
    deleteUser: (id: string) => Promise<void>,
}

export const useUserStore = create<UserStore>((set) => ({

    user: null,

    setUser: (user: User | null) => set({ user }),

    createUser: async (user: User) => {

        try {

            await api.post("/users/register-account", user)

        } catch (error: any) {

            console.error(error)
        }
    },

    updateUser: async (id: string, updateFields: any) => {

        try {

            await api.put(`/users/update-account/${id}`, updateFields)

        } catch (error: any) {

            console.error(error)
        }
    },

    deleteUser: async (id: string) => {

        try {

            await api.delete(`/users/delete-account/${id}`)

        } catch (error: any) {

            console.error(error)
        }
    }

}))
