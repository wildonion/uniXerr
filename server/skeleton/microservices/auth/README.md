



# Development Setup

* **Watch _auth_ microservice:** ```cargo watch -x run```

# User Entity API Docs

#### Required Fields for Signup: ```username + email + password + phone_number + device_id```

#### Required Fields for Login: ```username_or_eamil + password```

#### Required Fields for Edit User Data: ```username + phone_number + sex + age + email```

#### Required Fields for Edit User Profile Image: ```prof_img``` file

* **Get All Users with Access Level 2:** ```GET - /skeleton/api/auth/users```

* **Get a User with Access Level 1 or 2:** ```GET - /skeleton/api/auth/user/get/{id}```

* **Post a User with Access Level 2:** ```POST - /skeleton/api/auth/user/add```

* **Edit a User with Access Level 1 or 2:** ```POST - /skeleton/api/auth/user/edit/{id}```

* **Edit a User Password Access Level 1 or 2:** ```POST - /skeleton/api/auth/user/edit/password/{id}```

* **Edit a User Profile Image with Access Level 1 or 2:** ```POST - /skeleton/api/auth/user/edit/profile/{id}```

* **Get a User Profile Image with Access Level 1 or 2:** ```GET - /skeleton/api/auth/user/profile/{id}```

* **Delete a User with Access Level 1 or 2:** ```POST - /skeleton/api/auth/user/delete/{id}```

* **Register a New User:** ```POST - /skeleton/api/auth/register```

* **Login Registered User:** ```POST - /skeleton/api/auth/login```

* **Login Registered User:** ```POST - /skeleton/api/auth/login```

* **Check Token API:** ```POST - /skeleton/api/auth/check-token```

* **Logout API:** ```POST - /skeleton/api/auth/logout```

# User Friend Entity API Docs

#### Required Fields for Sending Request, Follow and Unfollow: ```from_user_id + to_friend_id```

* **Get All Friends of a User with Access Level 1 or 2:** ```GET - /skeleton/api/auth/user/get/{id}/friends```

* **Send Request to a Friend with Access Level 1 or 2:** ```POST - /skeleton/api/auth/user/send-request```

* **Follow a Friend with Access Level 1 or 2:** ```POST - /skeleton/api/auth/user/{user_id}/friend/{friend_id}/follow```

* **Unfollow a Friend with Access Level 1 or 2:** ```POST - /skeleton/api/auth/user/{user_id}/friend/{friend_id}/unfollow```

* **Delete a _user_friend_ Record with Access Level 2:** ``` POST - /skeleton/api/auth/user-friend/delete/{id}```

# Login History Entity API Docs

* **Get All Login Histories with Access Level 2:** ```GET - /skeleton/api/auth/login-histories```

* **Get a Login History with Access Level 2:** ```GET - /skeleton/api/auth/login-history/{id}```

* **Delete a Login History with Access Level 2:** ```POST - /skeleton/api/auth/login-history/delete/{id}```