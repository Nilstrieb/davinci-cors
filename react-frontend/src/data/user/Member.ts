import MemberRole from "./MemberRole";

export default interface Member {
    "user": string,
    "email"?: string,
    "class": string,
    "displayName": string
    "role": MemberRole
}