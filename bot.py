#!/usr/bin/python3
import discord
from discord.ext import commands
import os
from dotenv import load_dotenv
import mariadb
import sys
import random
import string
import asyncio
from argon2 import PasswordHasher

ph = PasswordHasher()
filepath = "siivagunner.txt"

def keygenfunc(length):
	hex_characters = string.hexdigits.lower()
	random_key = ''.join(random.choices(hex_characters, k=length))
	
	return random_key

def getrandomgranddad():
    try:
        with open(filepath, 'r') as file:
            lines = file.readlines()
        if not lines:
            return f"Error: The file '{filepath}' is empty."
        num_lines = len(lines)
        random_index = random.randint(0, num_lines - 1)
        random_line = lines[random_index].strip()
        line_number = random_index + 1
        return random_line

    except FileNotFoundError:
        return f"Error: The file '{filepath}' was not found."
    except Exception as e:
        return f"An unexpected error occurred: {e}"

def well_fuck_me_____with_a___():
	uncountablenouns = {"soap","shampoo","toothpaste","sand","water","soil","grass","bread","cheese","milk","butter","sugar","salt","flour","chocolate"}
	newadj = "newadj"
	objcts = "objects"

	with open(objcts, 'r') as file:
		objects = file.readlines()

	with open(newadj, 'r') as file:
		adjectives = file.readlines()

	adj_index = random.randint(0, len(adjectives) - 1)
	adj_line = adjectives[adj_index].strip()

	obj_index = random.randint(0, len(objects) - 1) # get random object from file with a big list
	obj_line = objects[obj_index].strip()

	aanornot = "a"
	if obj_line.endswith('s') or obj_line in uncountablenouns:
		aanornot = ""
	elif obj_line[0] not in 'aeiou':
		aanornot = "a "
	else:
		aanornot = "an "
		
	return "well fuck me " + adj_line + " with " + aanornot + obj_line


load_dotenv()
TOKEN = os.getenv('TOKEN')
DBPASS = os.getenv('DBPASS')

try:
	conn = mariadb.connect(
		user="usr",
		password=DBPASS,
		host="localhost",
		port=3306,
		database="appdb"

	)
except mariadb.Error as e:
	print(f"Error connecting to MariaDB Platform: {e}")
	sys.exit(1)

cur = conn.cursor()

intents = discord.Intents.default()
intents.message_content = True

client = commands.Bot(command_prefix='lsd-', help_command=None, intents=intents)

@client.event
async def on_ready():
	print(f'We have logged in as {client.user}')
	await client.change_presence(activity=discord.CustomActivity(name='Send "lsd-help" for a list of commands!'))

@client.command(name='key')
@commands.has_role("Moderator")
async def key(ctx, user: discord.Member):
	user_id = user.id
	cur.execute("SELECT discordid FROM users WHERE discordid = ?", (user_id,))
	existing_user = cur.fetchone()
	if existing_user:
		await ctx.send("User already has a key.")
		return
	user = await client.fetch_user(user_id)
	if user:
		try:
			await user.send("Use the following key to register in lsdblox:")
			key = keygenfunc(12)
			cur.execute(
				"INSERT INTO users (discordid,invkey) VALUES (?,?)",
				(user_id, key)
				)
			conn.commit()
			await user.send(key)
		except discord.Forbidden:
			await ctx.send("This member has DMs disabled!, so " + "<@" + user_id + ">" + ", please enable them so I can send you your key.")
			return
	else:
		ctx.send("User not found!")

@client.command(name='stat')
@commands.has_role("Moderator")
async def stat(ctx, *, stat):
	await ctx.send("Status changed to " + stat)
	await client.change_presence(activity=discord.CustomActivity(name=stat))

@client.command(name='resetpass')
@commands.has_role("Registered")
async def key(ctx):
	user = ctx.author
	user_id = user.id

	try:
		dm = await user.create_dm()
		await dm.send(
			"Welcome to the password reset form, " + user.name + ", Please send me the password you want to set as your new one. (15 chars minimum length)"
		)
	except discord.Forbidden:
		await ctx.send("This member has DMs disabled!, so " + "<@" + user_id + ">" + ", please enable them so I can help you reset your password.")
		return

	def checkpass(m):
		return m.author == user and m.channel == dm

	validpass = False
	while not validpass:
		try:
			newpasswordmsg = await client.wait_for('message', check=checkpass, timeout=60.0)
			newpassword = newpasswordmsg.content.strip()
			if len(newpassword) < 15:
				await dm.send(f"Your password is not long enough, please try again. (15 chars minimum length)")
				continue
			
			validpass = True
			encodepass = newpassword.encode('utf-8')
			hashed_password = ph.hash(encodepass)
			newuuid = keygenfunc(128)
			cur.execute(
				"UPDATE users SET pass = ? WHERE discordid = ?",
				(hashed_password, user_id)
				)
			conn.commit()
			cur.execute(
				"UPDATE users SET authuuid = ? WHERE discordid = ?",
				(newuuid, user_id)
				)
			conn.commit()
			await dm.send(f"Your new password has been carefully recorded, please delete your previous message containing it for security.")

		except asyncio.TimeoutError:
			await dm.send("Password reset form timed out (60 seconds). Please run the command again if you want to continue.")
			return

@client.command(name='granddad7')
@commands.has_role("Registered")
async def granddad7(ctx):
	user = ctx.author
	await ctx.send(getrandomgranddad())

@client.command(name='wellfmewa')
@commands.has_role("Registered")
async def granddad7(ctx):
	user = ctx.author
	await ctx.send(well_fuck_me_____with_a___())

@client.command(name='help')
async def commands(ctx):
	user = ctx.author
	await ctx.send(
		"-- List of commands --\n"
		"`lsd-resetpass` -- Password reset helper.\n"
		"`lsd-granddad7` -- FLINTSTONES? Sends a random SiIvagunner high quality rip.\n"
		"`lsd-wellfmewa` -- Say the line, bart! Sends a message with 'well fuck me' [modifier] 'with a/an' [object] \n"
		"Moderator only commands:\n"
		"`lsd-key @user` -- Sends a fellow member of the server a key so they can register.\n"
		"`lsd-stat text` -- Sets the status of the bot to whatever text.\n"
		)

client.run(TOKEN)
