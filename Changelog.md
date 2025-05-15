# Changelog

Like the captain's log from Star Trek except I'm not English or Bald, nor in space, nor a captain.

### May 14 2025

#### 12:41 PM

- Added a `folder_path` field to the UiComponent struct because, while I was cooking up components, I noticed that having everything be a sibling was created a big fat f-mess
- Decided to remove the `COMPONENT` from the component definition variable names - it's kinda redundant.
- Now that I say this outloud, writing out "literal" is probably also redundant.
- In fact, I am going to redeclare the component system to be an enum and declare the literals inside some kind of enum impl so I don't have to deal with this anymore.

#### 3:05 PM

- I did not declare the literals in-line, but we do have an enum impl for the atoms

### May 15 2025

#### 7:47 AM

- Moved main registry to components/atoms, will copy pattern across future molecules and features
- Added state enum for storing client-side state controllers
- Added decorators enum for storing purely decorative elements, currently, just horizontal/vertical separators, optionally with or without a label
- Cleaned up UI side, components are organized loosely as either an ATOM or a FEATURE component
- ATOMs are extracted, repeating components from FEATUREs
- FEATURE components compose ATOMs plus one-off layout + data to do WORK
- I love WORK
- Still not totally sure what to do with things like Utils or State
- They go in their own top level folder for now, but State might be a thing that gets moved to FEATUREs later

#### 9:08 AM

- Removed State from ATOMs, moved the only existing state, `AuthState` to new library/client/components/features/auth.rs
- Basic impl of auth in auth.rs, todo: Password reset form, password challenge form
- Todo: Think about adding SSO
- This thing builds, but a lot of code is unused
- Going to continue writing the Auth feature client side for now, copying over any improvements. Specifically thinking about structured fetch requests, click handlers, etc.
