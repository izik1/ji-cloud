insert into "user" (id, username, email, created_at, given_name, family_name, language, locale,
                    opt_into_edu_resources, over_18, timezone, organization)
values ('1f241e1b-b537-493f-a230-075cb16315be', 'test', 'test@test.test',
        '2020-08-08T00:11:21Z'::timestamptz, 'Bobby', 'Tables', 'en_US', 'en_US', true, true, 'US/Pacific-New', 'test org');

-- 1 is "Admin", 2 is "ManageCategory", 3 is "ManageImage", 4 is "ManageJig"
insert into "user_scope" (user_id, scope) values ('1f241e1b-b537-493f-a230-075cb16315be', 1), ('1f241e1b-b537-493f-a230-075cb16315be', 2), ('1f241e1b-b537-493f-a230-075cb16315be', 3), ('1f241e1b-b537-493f-a230-075cb16315be', 4), ('1f241e1b-b537-493f-a230-075cb16315be', 5), ('1f241e1b-b537-493f-a230-075cb16315be', 6);

-- password is 'password1'
insert into "user_auth_basic" (user_id, email, password) values ('1f241e1b-b537-493f-a230-075cb16315be', 'test@test.test', '$argon2id$v=19$m=8192,t=16,p=1$3f60oO10WmwVJ9MIFf1f6w$CcjLqbHaDP7cJXAut6S9cmgGg6NL2Jsg++aIpdvmaBg');
