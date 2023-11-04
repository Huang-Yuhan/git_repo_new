package com.example.spring.Controller;

import com.example.spring.Entity.MovieInfoEntity;
import com.example.spring.Service.MovieService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class MovieController {
    @Autowired
    private MovieService movieService;

    @RequestMapping("/getMovieInfo")
    public MovieInfoEntity getMovieInfo(@RequestParam("name") String name) {
        return movieService.getMovieInfo(name);
    }
}
