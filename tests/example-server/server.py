import os
import glob
from flask import Flask, request, send_file

app = Flask(__name__)

@app.route('/', methods=['GET'])
def root():
    return send_file("app.zonk")

@app.route('/articles/total', methods=['GET'])
def total_articles():
    return str(len(os.listdir("article")))

@app.route('/article/<path:path>', methods=['GET'])
def article_title(path):
    return send_file("article/" + path)

@app.route('/article/<int:id>/comments/total', methods=['GET'])
def total_comments(id):
    path = "article/" + str(id) + "/comment"
    if not os.path.exists(path):
        os.makedirs(path)
    return str(len(os.listdir(path)))

@app.route('/article/<int:id>/comments/add', methods=['GET', 'POST'])
def add_comment(id):
    path = "article/" + str(id) + "/comment"
    next_comment = len(os.listdir(path))
    file = open(path + '/' + str(next_comment), 'wb')
    file.write(request.data)
    file.close()
    return "Success"

@app.route('/article/<int:id>/width', methods=['GET'])
def article_width(id):
    with open("article/" + str(id) + "/width", 'r') as f:
        width = f.read().replace(' ', '').replace('\n', '')
        return width

@app.route('/clean', methods=['GET'])
def clean_comments():
    dirs = os.listdir("article")
    for id in range(len(dirs)):
        comments = glob.glob('article/' + str(id) + '/comment/*')
        for c in comments:
            print("Removing: " + c)
            os.remove(c)
    return "start { set_page(Page().add(Text(\"Successfully removed comments\"))); }"

if __name__ == '__main__':
    app.run(port=8000)
