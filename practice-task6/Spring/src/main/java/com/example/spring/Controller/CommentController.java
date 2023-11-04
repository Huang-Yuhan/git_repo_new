package com.example.spring.Controller;

import com.example.spring.Service.CommentService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.RestController;

import java.util.Map;

@RestController
public class CommentController {
    @Autowired
    private CommentService commentService;

    @RequestMapping("/comment")
    public String comment(@RequestBody Map<String,String> params)
    {
        String username = params.get("username");
        String movie_name = params.get("movie_name");
        String comment = params.get("commentDetail");
        commentService.commentMovie(username,movie_name,comment);
        return "success";
    }

}
