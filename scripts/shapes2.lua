PLUGIN_NAME = "shapes2"

SHAPE_POSITIONS = { 10, 10 }

function Update()
    SHAPE_POSITIONS[1] = SHAPE_POSITIONS[1] + 2
    SHAPE_POSITIONS[2] = SHAPE_POSITIONS[2] + 1
end

function Draw(screen)
    screen:draw_circle((screen.size.x / 2 - 50) + math.sin(SHAPE_POSITIONS[1] / 8) * 10, SHAPE_POSITIONS[1], 20, "green")
    screen:draw_rectangle((screen.size.x / 2 + 50) + math.sin(SHAPE_POSITIONS[1] / 8) * 10, SHAPE_POSITIONS[2], 40, 40, "pink")
end