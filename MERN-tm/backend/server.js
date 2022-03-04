const express = require('express')
const colors = require('colors')
const dotenv = require('dotenv').config()
const { errorHandler } = require('./middleware/errorMiddleware')
const connectDB = require('./config/db')
const port = process.env.PORT || 8000

connectDB()

const app = express()

// middelware to handle body data
app.use(express.json())
app.use(express.urlencoded({ extended: false }))

// route
app.use('/api/goals', require('./routes/goalRoutes'))
app.use('/api/users', require('./routes/userRoutes'))

// Override defualt err handler
app.use(errorHandler)

app.listen(port, () => console.log(`Server started on port ${port}`))