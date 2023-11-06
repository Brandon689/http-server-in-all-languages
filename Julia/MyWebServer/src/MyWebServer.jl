using HTTP

const ROOT = joinpath(dirname(dirname(pathof(MyWebServer))), "public")

function serve_file(path)
    full_path = joinpath(ROOT, path)
    if isfile(full_path)
        return HTTP.Response(200, read(full_path))
    else
        return HTTP.Response(404, "Not Found")
    end
end

function handler(req::HTTP.Request)
    # Default to index.html if the root is requested
    target = req.target == "/" ? "index.html" : req.target
    
    # Detect the file type and set the Content-Type header accordingly
    if occursin(".html", target)
        return serve_file(target) |> set_content_type("text/html")
    elseif occursin(".css", target)
        return serve_file(target) |> set_content_type("text/css")
    elseif occursin(".jpg", target) || occursin(".png", target)
        return serve_file(target) |> set_content_type("image/jpeg")
    else
        return HTTP.Response(404, "Not Found")
    end
end

function set_content_type(response, content_type)
    HTTP.setheader(response, "Content-Type" => content_type)
    return response
end

function main()
    server = HTTP.serve(handler, "localhost", 8080; verbose=true)
    println("Serving at http://localhost:8080")
    wait(server)
end

if abspath(PROGRAM_FILE) == abspath(@__FILE__)
    main()
end
