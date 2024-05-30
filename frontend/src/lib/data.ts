
import { Message } from "../store/MessageStore";
import { Instance, ROLE, User } from "./types"

export const users: User[] = [
    { id: "1u", created_at: "2023-01-15T10:20:30Z", name: "Alice Johnson", email: "alice.johnson@example.com", username: "AJohnson", role: ROLE.USER, n_instances: 2 },
    { id: "2u", created_at: "2022-11-20T14:25:45Z", name: "Luciano Revillod", email: "lrevillod@example.com", username: "lrevillod", role: ROLE.ADMINISTRATOR, n_instances: 0 },
    { id: "3u", created_at: "2023-03-05T08:15:00Z", name: "Carol White", email: "carol.white@example.com", username: "CWhite", role: ROLE.USER, n_instances: 1 },
    { id: "4u", created_at: "2023-02-17T11:45:30Z", name: "Albert Johnson", email: "albert.johnson@example.com", username: "AlbertJ", role: ROLE.USER, n_instances: 1 },
    { id: "5u", created_at: "2023-04-12T09:00:00Z", name: "Bart Sons", email: "b.sons@example.com", username: "BSons", role: ROLE.USER, n_instances: 1 },
    { id: "6u", created_at: "2022-12-30T16:30:15Z", name: "Carl Brown", email: "carl.b@example.com", username: "CBrown", role: ROLE.USER, n_instances: 2 },
    { id: "7u", created_at: "2023-01-10T13:20:50Z", name: "David Brown", email: "david.brown@example.com", username: "DBrown", role: ROLE.USER, n_instances: 1 },
    { id: "8u", created_at: "2023-03-22T07:45:35Z", name: "Tina Allen", email: "tina.allen@example.com", username: "TAllen", role: ROLE.USER, n_instances: 1 },
    { id: "9u", created_at: "2023-04-05T18:50:10Z", name: "Martin King", email: "m.king@example.com", username: "MKing", role: ROLE.USER, n_instances: 1 },
    { id: "10u", created_at: "2023-02-28T20:35:25Z", name: "Second Martin Queen", email: "m.queen@example.com", username: "MQueen", role: ROLE.USER, n_instances: 0 },
    { id: "11u", created_at: "2023-01-15T10:20:30Z", name: "Second Alice Johnson", email: "alice.johnson@example.com", username: "AJohnson", role: ROLE.USER, n_instances: 2 },
    { id: "21u", created_at: "2022-11-20T14:25:45Z", name: "Second Luciano Revillod", email: "lrevillod@example.com", username: "lrevillod", role: ROLE.ADMINISTRATOR, n_instances: 0 },
    { id: "31u", created_at: "2023-03-05T08:15:00Z", name: "Second Carol White", email: "carol.white@example.com", username: "CWhite", role: ROLE.USER, n_instances: 1 },
    { id: "41u", created_at: "2023-02-17T11:45:30Z", name: "Second Albert Johnson", email: "albert.johnson@example.com", username: "AlbertJ", role: ROLE.USER, n_instances: 1 },
    { id: "51u", created_at: "2023-04-12T09:00:00Z", name: "Second Bart Sons", email: "b.sons@example.com", username: "BSons", role: ROLE.USER, n_instances: 1 },
    { id: "61u", created_at: "2022-12-30T16:30:15Z", name: "Second Carl Brown", email: "carl.b@example.com", username: "CBrown", role: ROLE.USER, n_instances: 2 },
    { id: "71u", created_at: "2023-01-10T13:20:50Z", name: "Second David Brown", email: "david.brown@example.com", username: "DBrown", role: ROLE.USER, n_instances: 1 },
    { id: "81u", created_at: "2023-03-22T07:45:35Z", name: "Second Tina Allen", email: "tina.allen@example.com", username: "TAllen", role: ROLE.USER, n_instances: 1 },
    { id: "91u", created_at: "2023-04-05T18:50:10Z", name: "Second Martin King", email: "m.king@example.com", username: "MKing", role: ROLE.USER, n_instances: 1 },
    { id: "110u", created_at: "2023-02-28T20:35:25Z", name: "Martin Queen", email: "m.queen@example.com", username: "MQueen", role: ROLE.USER, n_instances: 0 },
]

export const instances: Instance[] = [
    { id: "1i", type: "Container", status: "Running", name: "My Instance 1", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "1u" },
    { id: "2i", type: "VM", status: "Running", name: "My Instance 2", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "1u" },
    { id: "3i", type: "Container", status: "Stopped", name: "My Instance 3", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "3u" },
    { id: "4i", type: "Container", status: "Running", name: "My Instance 4", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "4u" },
    { id: "5i", type: "Container", status: "Rebooting", name: "My Instance 5", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "3", user_id: "5u" },
    { id: "6i", type: "VM", status: "Running", name: "My Instance 6", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "6u" },
    { id: "7i", type: "Container", status: "Stopped", name: "My Instance 7", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "6u" },
    { id: "8i", type: "VM", status: "Running", name: "My Instance 8", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "3", user_id: "7u" },
    { id: "9i", type: "Container", status: "Running", name: "My Instance 9", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "8u" },
    { id: "10i", type: "Container", status: "Running", name: "My Instance 10", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "9u" },
    { id: "11i", type: "Container", status: "Running", name: "My Instance 1", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "1u" },
    { id: "21i", type: "VM", status: "Running", name: "My Instance 2", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "1u" },
    { id: "31i", type: "Container", status: "Stopped", name: "My Instance 3", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "3u" },
    { id: "41i", type: "Container", status: "Running", name: "My Instance 4", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "4u" },
    { id: "51i", type: "Container", status: "Rebooting", name: "My Instance 5", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "3", user_id: "5u" },
    { id: "61i", type: "VM", status: "Running", name: "My Instance 6", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "6u" },
    { id: "71i", type: "Container", status: "Stopped", name: "My Instance 7", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "6u" },
    { id: "81i", type: "VM", status: "Running", name: "My Instance 8", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "3", user_id: "7u" },
    { id: "91i", type: "Container", status: "Running", name: "My Instance 9", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "2", user_id: "8u" },
    { id: "110i", type: "Container", status: "Running", name: "My Instance 10", ip_addresses: ["127.0.0.2"], specs: { cpu: 2, ram: 4, storage: 100 }, cluster_node: "1", user_id: "9u" }
]

export const messages: Message[] = [
    { id: "1", from: "John Doe", message: "Hello, how are you?", date: "2021-09-01" },
    { id: "2", from: "Jane Doe", message: "I'm fine, thank you", date: "2021-09-02" },
    { id: "3", from: "John Doe", message: "I'm glad to hear that", date: "2021-09-03" },
    { id: "4", from: "Jane Doe", message: "How about you?", date: "2021-09-04" },
    { id: "5", from: "John Doe", message: "I'm fine too", date: "2021-09-05" },
    { id: "6", from: "Jane Doe", message: "That's great", date: "2021-09-06" },
    { id: "7", from: "John Doe", message: "Yes, it is", date: "2021-09-07" },
    { id: "8", from: "Jane Doe", message: "I have to go now", date: "2021-09-08" },
    { id: "9", from: "John Doe", message: "Ok, see you later", date: "2021-09-09" },
    { id: "10", from: "Jane Doe", message: "Bye", date: "2021-09-10" },
    { id: "11", from: "Second John Doe", message: "Hello, how are you?", date: "2021-09-01" },
    { id: "21", from: "Second Jane Doe", message: "I'm fine, thank you", date: "2021-09-02" },
    { id: "31", from: "Second John Doe", message: "I'm glad to hear that", date: "2021-09-03" },
    { id: "41", from: "Second Jane Doe", message: "How about you?", date: "2021-09-04" },
    { id: "51", from: "Second John Doe", message: "I'm fine too", date: "2021-09-05" },
    { id: "61", from: "Second Jane Doe", message: "That's great", date: "2021-09-06" },
    { id: "71", from: "Second John Doe", message: "Yes, it is", date: "2021-09-07" },
    { id: "81", from: "Second Jane Doe", message: "I have to go now", date: "2021-09-08" },
    { id: "91", from: "Second John Doe", message: "Ok, see you later", date: "2021-09-09" },
    { id: "10", from: "Second Jane Doe", message: "Bye", date: "2021-09-10" }
]