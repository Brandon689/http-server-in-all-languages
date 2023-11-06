from flask import Flask, render_template

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('index.html')

    # Define a new route
@app.route('/about')
def about():
    return "This is the About page."

# Define another route with a dynamic URL parameter
@app.route('/user/<username>')
def user_profile(username):
    return f"Hello, {username}! This is your user profile."

if __name__ == '__main__':
    app.run(debug=True)
