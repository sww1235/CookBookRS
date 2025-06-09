# CookBookRS

This is a rewrite in rust of my initial attempt at this project in
[golang](https://github.com/sww1235/CookBook)

This is a recipe database and viewer designed to easily store recipes, help
generate shopping lists and meal recommendations, and eventually connect to a
kitchen inventory database to track items in your pantry and allow you to cook
with what you have.

## Data Storage and File Formats

Recipes are stored as TOML files, with the following general format.

> [!WARNING]
> The UUIDs shown in the example below are examples and should not be used for real life applications.

> [!WARNING]
> If using the PostGreSQL features, the UUIDs of ingredients and equipment must be identical to the respective primary key in the database.

```toml
# Optional. If not defined in file, will be defined and written out when files are saved.
id = '1ae4f773-e08a-4a5d-b8bc-6be9404269aa'
name = "Recipe Name"
# Optional. This supports newlines so a multi-line string is acceptable here instead
description = "This is a description."
# Optional. This supports newlines so a multi-line string is acceptable here instead
comments = "Here is a comment."
# The source should be a descriptive reference of where the recipe was found. If unknown, put Unknown.
source = "Where this recipe was from."
# The author should be the name of the author of the recipe if known. If unknown, put Unknown
author = "Author of Recipe"
# the numerical quantity the recipe makes. The "5" in "5 cups". The units are specified in amount_made_units
amount_made = 5
# the units counting how much the recipe makes. The "cups" in "5 cups". The numerical quantity is specified in amount_made. This is not parsed currently.
amount_made_units = "cups"
# This is a TOML array. All tag definitions are parsed as strings.
tags = ["tag1", "tag2"]

# Include a [[steps]] block for each step in a recipe.
# Each ingredient  and equipment block must be specified below its respective step per the TOML specifications.

[[steps]]
id = '628c0a92-44e4-4d92-93b1-21c3aa391592'
# Optional. Specified in Seconds.
time_needed = 300.5
# Optional. Specified in Kelvin
temperature = 400.2
instructions = "Example Step Instructions"
# Step Type should be selected from the following list: ["Prep", "Cook", "Wait", "Other"].
step_type = "Other"

# Repeat this for each ingredient in a step
[[steps.ingredients]]
# This is a database key.
id = '03f5f051-fbe4-494c-ba97-88ed914a5b1b'
name = "Ingredient Name"
# Optional. This supports newlines so a multi-line string is acceptable here instead
description = "This is a description."

# Only specify one of Quantity, Mass or Volume
# Quantity represents a count or physical quantity of an Ingredient
# Ex: 30 chocolate chips
# Mass represents the mass of an Ingredient, specified in grams
# Volume represents the volume of an Ingredient, specified in m^3
unit_quantity = {Quantity = 25.4}

# repeat this for each piece of equipment in a step
[[steps.equipment]]
# This is a database key.
id = '47b7c070-c89a-4c39-abd1-a3a416b0d04f'
name = "Equipment Name"
# Optional. This supports newlines so a multi-line string is acceptable here instead
description = "This is a description."
is_owned = false



```


An optional database for ingredient inventory can be specified via the config file. This is a postgreSQL database with the following schema:

SCHEMA TBD
