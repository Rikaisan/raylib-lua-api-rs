PLUGIN_NAME = "shapes"

SHAPE_POSITIONS = { 10, 10 }

function Update()
    SHAPE_POSITIONS[1] = SHAPE_POSITIONS[1] + 1
    SHAPE_POSITIONS[2] = SHAPE_POSITIONS[2] + 2
end

function Draw(screen)
    screen:draw_circle(SHAPE_POSITIONS[1], screen.size.y / 2 - 100, 20, "blue")
    screen:draw_rectangle(SHAPE_POSITIONS[2], screen.size.y / 2 + 100, 40, 40, "red")
end