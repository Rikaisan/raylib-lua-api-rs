PLUGIN_NAME = "circles"

CIRCLE_POS = { 10, 10 }

function Update()
    CIRCLE_POS[1] = CIRCLE_POS[1] + 1
    CIRCLE_POS[2] = CIRCLE_POS[2] + 2
end

function Draw(screen)
    screen:draw_circle(CIRCLE_POS[1], screen.size.y / 2 - 100, 20, "blue")
    screen:draw_circle(CIRCLE_POS[2], screen.size.y / 2 + 100, 20, "red")
end