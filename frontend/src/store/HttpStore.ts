
import { create } from "zustand"

interface HttpStore {
    status: number,
    message: string,
    data: any | null,
}

export const useHttpStore = create<HttpStore>(() => ({
    status: 0,
    message: "",
    data: {},
}))
