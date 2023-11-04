package com.example.spring.Service;

import com.example.spring.Entity.MovieInfoEntity;
import com.example.spring.Repository.movie_infoRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class MovieService {
    @Autowired
    private movie_infoRepository movieRepos;
    public MovieInfoEntity getMovieInfo(String name) {
        return movieRepos.getByName(name);
    }
}
