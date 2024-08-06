class Rectangle
{
 public:
    Rectangle(int _width, int _height) : width(_width), height(_height)
    {
        // Do nothing
    }

    int get_width() const
    {
        return width;
    }

    int get_height() const
    {
        return height;
    }

    void set_width(int width)
    {
        if (width <= 0 || width > 1000)
        {
            return;
        }

        this->width = width;
    }

    void set_height(int height)
    {
        if (height <= 0 || height > 2000)
        {
            return;
        }

        this->height = height;
    }

    int area() const
    {
        return width * height;
    }

    int perimeter() const
    {
        return 2 * (width + height);
    }

    bool is_square() const
    {
        return width == height;
    }

 private:
    int width, height;
};