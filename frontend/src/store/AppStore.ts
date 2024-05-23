
import { create } from "zustand"
import { persist, createJSONStorage } from 'zustand/middleware'
import { toUpperAndLower } from "../lib/string"

interface AppStore {
    pageTitle: string,
    setPageTitle: (pageTitle: string) => void
}

const getInitialPageTitle = () => {

    const path = window.location.pathname.split("/")[1]

    const pages = ["users", "analytics", "instances", "dashboard"]

    if (pages.includes(path)) {
        return toUpperAndLower(path)
    }

    return ""
}

export const useAppStore = create(

    persist<AppStore>((set) => ({

        pageTitle: getInitialPageTitle(),
        setPageTitle: (pageTitle: string) => set({ pageTitle }),
    }),

        {
            name: 'app-store',
            storage: createJSONStorage(() => localStorage)
        },
    ),
)
