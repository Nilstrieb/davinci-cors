use crate::actions::Pool;
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::error::{ServiceErr, ServiceResult};
use crate::models::{
    Class, Guild, Member, MemberRole, NewClass, NewGuild, NewMember, Timetable, User,
};
use crate::schema::classes::dsl::*;
use diesel::{
    delete, insert_into, update, BoolExpressionMethods, ExpressionMethods, SaveChangesDsl,
};
use uuid::Uuid;

pub fn insert_class(db: &Pool, new_class: NewClass) -> ServiceResult<Class> {
    let conn = db.get()?;

    Ok(insert_into(classes).values(&new_class).get_result(&conn)?)
}

pub type ClassMemberData = (Class, Vec<(Member, User)>);

pub fn get_class(db: &Pool, class_id: Uuid) -> ServiceResult<Option<ClassMemberData>> {
    use crate::schema::members::dsl::{display_name, members, role};
    use crate::schema::users::dsl::users;
    let conn = db.get()?;

    let vec = classes
        .inner_join(members.inner_join(users))
        .filter(id.eq(class_id).and(role.ne(MemberRole::PENDING)))
        .order_by((role, display_name))
        .load(&conn)?;

    Ok(map_class_join_members(vec))
}

pub fn get_classes_by_user(db: &Pool, user_id: Uuid) -> ServiceResult<Vec<Class>> {
    use crate::schema::members::dsl::{members, role, user as member_user};
    let conn = db.get()?;

    let class_vec: Vec<(Class, Member)> = classes
        .inner_join(members)
        .filter(
            member_user
                .eq(user_id)
                .and(role.ne(MemberRole::BANNED))
                .and(role.ne(MemberRole::PENDING)),
        )
        .load(&conn)?;

    Ok(class_vec.into_iter().map(|(class, _)| class).collect())
}

pub fn get_pending_members(db: &Pool, class_id: Uuid) -> ServiceResult<Vec<Member>> {
    use crate::schema::members::dsl::{class, members, role};
    let conn = db.get()?;

    Ok(members
        .filter(class.eq(class_id).and(role.eq(MemberRole::PENDING)))
        .load(&conn)?)
}

pub fn get_banned_members(db: &Pool, class_id: Uuid) -> ServiceResult<Vec<(Member, User)>> {
    use crate::schema::members::dsl::{class, members, role};
    use crate::schema::users::dsl::users;
    let conn = db.get()?;

    Ok(members
        .inner_join(users)
        .filter(class.eq(class_id).and(role.eq(MemberRole::BANNED)))
        .load(&conn)?)
}

pub fn update_member(db: &Pool, member: NewMember) -> ServiceResult<Member> {
    let conn = db.get()?;

    Ok(member.save_changes(&*conn)?)
}

pub fn create_member(db: &Pool, member: NewMember) -> ServiceResult<Member> {
    use crate::schema::members::dsl::members;
    let conn = db.get()?;

    Ok(insert_into(members).values(&member).get_result(&conn)?)
}

pub fn update_class(db: &Pool, new_class: NewClass) -> ServiceResult<Class> {
    let conn = db.get()?;

    Ok(update(classes)
        .filter(id.eq(new_class.id))
        .set((
            name.eq(new_class.name),
            description.eq(new_class.description),
        ))
        .get_result(&conn)?)
}

pub fn set_discord_id_class(db: &Pool, class_id: Uuid, d_id: Option<&str>) -> ServiceResult<Class> {
    let conn = db.get()?;

    Ok(update(classes)
        .filter(id.eq(class_id))
        .set(discord_id.eq(d_id))
        .get_result(&conn)?)
}

pub fn get_class_by_discord(db: &Pool, class_id: &str) -> ServiceResult<Class> {
    let conn = db.get()?;

    classes
        .filter(discord_id.eq(class_id))
        .load(&conn)?
        .into_iter()
        .next()
        .ok_or(ServiceErr::NotFound)
}

pub fn get_member(db: &Pool, user_id: Uuid, class_id: Uuid) -> ServiceResult<(Member, User)> {
    use crate::schema::members::dsl::{class, members, user};
    use crate::schema::users::dsl::users;
    let conn = db.get()?;

    Ok(members
        .inner_join(users)
        .filter(class.eq(class_id).and(user.eq(user_id)))
        .get_result(&conn)?)
}

pub fn delete_member(db: &Pool, user_id: Uuid, class_id: Uuid) -> ServiceResult<usize> {
    use crate::schema::members::dsl::{class, members, user};
    let conn = db.get()?;

    Ok(delete(members.filter(class.eq(class_id).and(user.eq(user_id)))).execute(&conn)?)
}

pub fn delete_class(db: &Pool, class_id: Uuid) -> ServiceResult<usize> {
    let conn = db.get()?;

    Ok(delete(classes).filter(id.eq(class_id)).execute(&conn)?)
}

pub fn get_timetable(db: &Pool, class_id: Uuid) -> ServiceResult<Timetable> {
    use crate::schema::timetables::dsl::*;
    let conn = db.get()?;

    Ok(timetables.find(class_id).get_result(&conn)?)
}

pub fn create_timetable(db: &Pool, class_id: Uuid) -> ServiceResult<Timetable> {
    use crate::schema::timetables::dsl::*;
    let conn = db.get()?;

    Ok(insert_into(timetables)
        .values(class.eq(class_id))
        .get_result(&conn)?)
}

pub fn update_timetable(db: &Pool, new_timetable: Timetable) -> ServiceResult<Timetable> {
    use crate::schema::timetables::dsl::*;
    let conn = db.get()?;

    Ok(update(timetables)
        .filter(class.eq(new_timetable.class))
        .set(timetable.eq(new_timetable.timetable))
        .get_result(&conn)?)
}

pub fn delete_timetable(db: &Pool, class_id: Uuid) -> ServiceResult<usize> {
    use crate::schema::timetables::dsl::*;
    let conn = db.get()?;

    Ok(delete(timetables)
        .filter(class.eq(class_id))
        .execute(&conn)?)
}

pub fn insert_guild(db: &Pool, guild: NewGuild) -> ServiceResult<Guild> {
    use crate::schema::guilds::dsl::guilds;
    let conn = db.get()?;
    Ok(insert_into(guilds).values(guild).get_result(&conn)?)
}

pub fn change_guild_settings(db: &Pool, guild: NewGuild) -> ServiceResult<Guild> {
    let conn = db.get()?;
    Ok(guild.save_changes(&*conn)?)
}

pub fn get_guild_settings(db: &Pool, guild_id: &str) -> ServiceResult<Guild> {
    use crate::schema::guilds::dsl::guilds;

    let conn = db.get()?;
    Ok(guilds.find(guild_id).get_result(&conn)?)
}

pub fn map_class_join_members(vec: Vec<(Class, (Member, User))>) -> Option<ClassMemberData> {
    match vec
        .into_iter()
        .fold((None, vec![]), |(_, mut vec), (class, member)| {
            vec.push(member);
            (Some(class), vec)
        }) {
        (Some(class), vec) => Some((class, vec)),
        (None, _) => None,
    }
}
